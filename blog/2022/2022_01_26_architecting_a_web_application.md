+++
date = 2022-01-26T06:00:00Z
title = "Clean and Scalable Architecture for Web Applications in Rust"
type = "post"
tags = ["rust", "programming", "tutorial", "web"]
authors = ["Sylvain Kerkour"]
url = "/rust-web-application-clean-architecture"

[extra]
lang = "en"

comment ="""


"""
+++



There are many architectural patterns to design web applications. A famous one is the "[Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)" by *Robert C. Martin*

![The CLean Architecture - [source](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)](https://kerkour.com/2022/web-application-architecture/ch10_clean_architecture.jpg)

This architecture splits projects into different layers in order to produce systems that are
1. *Independent of Frameworks. The architecture does not depend on the existence of some library of feature laden software. This allows you to use such frameworks as tools, rather than having to cram your system into their limited constraints.*
2. *Testable. The business rules can be tested without the UI, Database, Web Server, or any other external element.*
3. *Independent of UI. The UI can change easily, without changing the rest of the system. A Web UI could be replaced with a console UI, for example, without changing the business rules.*
4. *Independent of Database. You can swap out Oracle or SQL Server, for Mongo, BigTable, CouchDB, or something else. Your business rules are not bound to the database.*
5. *Independent of any external agency. In fact your business rules simply don’t know anything at all about the outside world.*

You can learn more about the clean architecture in the eponym book: [Clean Architecture](https://www.goodreads.com/book/show/18043011-clean-architecture) by *Robert C. Martin*.


But, in my opinion, the so called "clean architecture" is **too complex**, with its jargon that resonates only with professional architects and too many layers of abstraction. It's not for people actually writing code.

This is why I propose another approach, equally flexible but much simpler and which can be used for traditional server-side rendered web applications and for JSON APIs.


> **This post contains excerpts from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)**

![Server's architecture](https://kerkour.com/2022/web-application-architecture/ch10_webapp_architecture.png)


As far as I know, this architecture has no official and shiny name, but I have used it with success for projects exceeding tens of thousands of lines of code in Rust, Go, and Node.JS.


The advantage of using such architecture is that, if in the future the requirements or one dependency are revamped, changes are locals and isolated.


Each layer should communicate only with adjacent layers.

Let's dig in!



### Presentation

The presentation layer is responsible for the deserialization of the requests and the serialization of the responses.

It has its own models (HTML templates or structure to be encoded in JSON / XML). It encapsulates all the details about encoding responses of our web server.

The presentation layer calls the services layer.

### Services

The services layer is where the business logic lives. All our application's rules and invariants live in the services layer.

Need to verify a phone number? But what is the format of a phone number? The response to this question is in the service layer.


What are the validations to proceed to when creating a job for an agent? This is the role of the service layer.



### Entities

The entities layer encapsulates all the structures that will be used by the services layer. Each service has its own group of entities.

Why not call this part a model? Because a model often refers to an object persisted in a database or sent by the presentation layer. In addition to being confusing, in the real world, not all entities are persisted. For example, an object representing a group with its users may be used in your services but neither persisted nor transmitted by the presentation layer.

In our case, the entities will `Agent`, `Job` (a job is a command created by the client, stored and dispatched by the server, and executed by the agent),


### Repository

The repository layer is a thin abstraction over the database. It encapsulates all the database calls.

The repository layer is called by the services layer.


{{< subscribe_form >}}


### Drivers

And the last piece of our architecture, `drivers`. Drivers encapsulate calls to third-party APIs and communication with external services such as email servers or block storage.

We use `interfaces` (or `traits` in Rust terms) to be able to switch from a specific driver implementation to another one fulfilling the same contract, at will.

`drivers` can only be called by `services`, because this is where the business logic lives.




## Scaling the architecture

You may be wondering, "Great, but how to scale our app once we already have a lot of features implemented and we need to add more?"

You simply need to "horizontally scale" your services and repositories. One pair for each [bounded domain context](https://martinfowler.com/bliki/BoundedContext.html).

![Scaling our architecture](https://kerkour.com/2022/web-application-architecture/ch10_scaling_webapp_architecture.png)

As you may have guessed, if our project or team becomes too big for a monolith, each service may become an independent "micro-service".


## Talk is cheap. Show me the code.

You can find a large web application using this architecture on GitHub: [github.com/skerkour/bloom-legacy](https://github.com/skerkour/bloom-legacy/tree/v3/bloom).

As I know that it's not easy to navigate thousands of lines of code of a codebase you have no prior experience with, I've extracted here a few lines of code for each layer.

Also, you can refer to this previous post to learn which crates I'm using: [Rust for web development: 2 years later](https://kerkour.com/rust-for-web-development-2-years-later/).


**drivers/db.rs**
```rust
pub type DB = Pool<Postgres>;

pub trait Queryer {
  // fn get()...
  // fn select()...
  // fn execute()...
}
```


**entities.rs**
```rust
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct File {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,
    pub size: i64,
    pub r#type: String,
    pub explicitly_trashed: bool,
    pub trashed_at: Option<DateTime<Utc>>,

    pub namespace_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
}
```

**repository/mod.rs**
```rust
#[derive(Debug)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}
```

**repository/create_file.rs**
```rust
impl Repository {
    pub async fn create_file(&self, db:  &dyn db::Queryer, file: &File) -> Result<(), Error> {
        const QUERY: &str = "INSERT INTO files
        (id, created_at, updated_at, name, size, type, explicitly_trashed, trashed_at, namespace_id, parent_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)";

        match sqlx::query(QUERY)
            .bind(file.id)
            .bind(file.created_at)
            .bind(file.updated_at)
            .bind(&file.name)
            .bind(file.size)
            .bind(&file.r#type)
            .bind(file.explicitly_trashed)
            .bind(file.trashed_at)
            .bind(file.namespace_id)
            .bind(file.parent_id)
            .execute(db)
            .await
        {
            Err(err) => {
                log::error!("files.create_file: Inserting file: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
```

**service/mod.rs**
```rust
#[derive(Debug)]
pub struct Service {
    repo: Repository,
    db: db::DB,
    storage: Arc<dyn drivers::Storage>,
    kernel_service: Arc<kernel::Service>,
}

impl Service {
    pub fn new(kernel_service: Arc<kernel::Service>, db: db::DB, storage: Arc<dyn drivers::Storage>) -> Service {
        let repo = Repository::new();
        Service {
            db,
            repo,
            storage,
            kernel_service,
        }
    }
}
```

You may ask: Why put the `db` in the `Service` and not in the repository if it's the repository that interacts with the database? Because we will sometimes need to call the repository's methods within transactions. So we need the repository's methods to be generics over 'normal' database operations and transactions.

> **This post contains excerpts from my book [Black Hat Rust](https://kerkour.com/black-hat-rust)**

**service/create_folder.rs**
```rust
#[derive(Debug, Clone)]
pub struct CreateFolderInput {
    pub parent_id: Uuid,
    pub name: String,
}

impl Service {
    pub async fn create_folder(&self, actor: Actor, input: CreateFolderInput) -> Result<File, kernel::Error> {
        let actor = self.kernel_service.current_user(actor)?;

        let parent = self.repo.find_file_by_id(&self.db, input.parent_id).await?;

        if parent.namespace_id.is_none() {
            return Err(Error::FileNotFound.into());
        }

        self.kernel_service
            .check_namespace_membership(&self.db, &actor, parent.namespace_id.unwrap())
            .await?;

        // valdiate input
        if parent.trashed_at.is_some() {
            return Err(Error::FolderIsInTrash.into());
        }

        match self
            .repo
            .find_file_by_parent_and_name(&self.db, parent.id, &input.name)
            .await
        {
            Ok(_) => return Err(Error::FileAlreadyExists.into()),
            Err(Error::FileNotFound) => {}
            Err(err) => return Err(err.into()),
        };

        self.validate_file_name(&input.name)?;

        let now = Utc::now();
        let file = File {
            id: Ulid::new().into(),
            created_at: now,
            updated_at: now,

            name: input.name,
            size: 0,
            r#type: consts::FILE_TYPE_FOLDER.to_string(),
            explicitly_trashed: false,
            trashed_at: None,

            namespace_id: parent.namespace_id,
            parent_id: Some(parent.id),
        };
        self.repo.create_file(&self.db, &file).await?;

        Ok(file)
    }
}
```

**server/api/model.rs**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolder {
    pub parent_id: Id,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: Id,
    pub created_at: Time,
    pub updated_at: Time,
    pub name: String,
    pub size: i64,
    pub r#type: String,
    pub explicitly_trashed: bool,
    pub trashed_at: Option<Time>,

    pub path: Vec<FilePath>,
    pub children: Option<Vec<File>>,
}

impl From<entities::File> for File {
    fn from(file: entities::File) -> Self {
        File {
            id: file.id,
            created_at: file.created_at,
            updated_at: file.updated_at,
            name: file.name,
            size: file.size,
            r#type: file.r#type,
            explicitly_trashed: file.explicitly_trashed,
            trashed_at: file.trashed_at,
            children: None,
            path: Vec::new(),
        }
    }
}
```

**server/api/commands/create_folder.rs**
```rust
pub async fn create_folder(
    ctx: web::Data<Arc<ServerContext>>,
    input: Json<input::CreateFolder>,
    actor: Actor,
) -> Result<api::Response<File>, kernel::Error> {
    let input = input.into_inner();
    let service_input = service::CreateFolderInput {
        parent_id: input.parent_id,
        name: input.name,
    };
    let file = ctx.files_service.create_folder(actor, service_input).await?;

    Ok(api::Response::ok(file.into()))
}
```


**Want to learn more real-world Rust development tips and tricks? Get my book [Black Hat Rust](https://kerkour.com/black-hat-rust).**
