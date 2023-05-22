+++
date = 2021-08-12T14:00:00Z
title = "Overthinking"
type = "post"
tags = ["entrepreneurship", "programming", "motivation", "mindset", "productivity"]
authors = ["Sylvain Kerkour"]
url = "/overthinking"

[extra]
lang = "en"

comment ="""
<!--

Commencer par dire pourquoi overthinking est nul (le why)

In your opinion, why all those smart kids out of the best engineering schools end up in bureaucratic 9 to 5 jobs they will hate most of their career?

i've tracked dowb this issue from 3 sources. 3 Poisons from where overthinking originate all the pain. 3 Poisons that leads to toxic mental states that cloud the mind.

teachers are rarely great achievers. Because their role is to tech us the

the right answer

the thing is: there is rarely one true thing

Im not saying that ypu shoyld turn off yoyr brain today, I'm simply arguing that the thing that will bring ypu the more result may not be the one you are thinking about

trust yoyr vision and stop trying to connect all tge dot ahead of time (lien vers conf steve jobs)


if you are going from 0 to 1, emvrace uncertainty and that many thing are out of yoyr reach and knowledge bow if you are going from 10 to 100 it's another story, but at this point only you know what needs to be dones

-->
"""
+++




I used to be a pathological overthinker. Everything had to be carefully planned, from the words I would say to the baker to order a baguette to all the insignificant details of any of the projects I worked on. Is it better to tweet at 18:00 or 18:05? Rust or Go? Functional or imperative? Federated or peer-to-peer? `FROM scratch` or `FROM alpine`? Open source or free software? Blue or green?

Is even thinking about overthinking, overthinking? 🤯

Then I realized that there is another expression to describe this situation: **analysis paralysis**.

We all know that *"The perfect is the enemy of the good."*, still we lose countless hours trying to make the perfect design, the perfect presentation, the perfect specification document, the perfect data model, chose the perfect technology or framework.

The result? Projects never materialize or are months behind schedule, and everybody is frustrated.

I've tracked down the sources of this problem to 3 causes. 3 poisons from where all the pain originates. These 3 poisons are:
* Education
* Marketing
* Ego


## 1st poison: Education

<!-- Educational programs are designed to teach us the "one truth". THE right answer. But in life, there is no single truth, there is only approximations and negotiations. -->

Education is rarely about **doing**. It's mostly about **thinking**. Thus we learn how to think, and we end up with whole countries filled with people who think, think, think but never start doing anything. The result is a lot of debates, whether it be online or on the TV, and stupid & complex rules and bureaucracies coming from people in need to justify all the time they are paid to think, think, think.

In the tech world, this is flagrant when reading some blogs about software architecture. The terms used are so far from the reality of the field that it's close to absurd. My take on the matter is that the architect of any project needs to be one of the developers, one of the *doers*.


## 2nd Poison: Marketing

The second poison is marketing pushed by Business to Developers (B2D) companies.

These companies, while solving real problems (often something among the lines of scalability or data processing), have great interests to make us believe that we have the problems they are solving and that they are our best hope to solve those problems (we most of the time don't have these problems).

So now, when designing our systems, we no longer consider only the simplest solution, but a myriad of alternatives in order to solve our future, imaginary, problems. SQL or NoSQL? Containers or Functions? Java or Kotlin?

Let's be honest, most of the time (I won't invent some made up stat here, but I suspect it's over 90%), we only need [PostgreSQL](https://www.postgresql.org/) for the database, Go, PHP (Laravel) or Node.js for the backend, and React or Vue.js for the frontend. Pick according to the tastes of the team.

Why PostgreSQL? Because most cloud providers have a managed PostgreSQL offer, so you can pick any of them while being sure than your data won't disappear because of an SSD failure.


## 3rd Poison: Ego

We all fear being judged by our work. When it's your first or second launch, it really hurts when some random person on internet forums start pointing out this little, irrelevant thing that you spent 10 minutes on, among the hundreds to thousands of hours on your project, and start making some drama about how bad you are as a developer, blogger or person.


Thus, to anticipate critics, we start spending time polishing the details. More and more time. One day we realize that all this time flew polishing details while the important points, those bringing real value, are not even started.

We fear making mistakes and then being publicly pointed and shamed for those mistakes. By shipping and publicly sharing multiple projects, I've experienced that the people worth impressing will never call you on small mistakes. They understand that we are all humans: fallible, inexperienced, imperfect. All the cynical nitpicks were never interested in my projects, they are here only to tell the world how they are smarter and greater than us. They are a vocal minority and it's almost never worth our time and attention.


I've also noticed that, up to a certain point, the smarter a person is, the more it has to be apparent in their work. Every algorithm needs to be perfect, every function needs to be side-effects free, every data structure needs to be the fastest, and every best practice needs to be followed. It's not bad per se, but when it prevents doing the real stuff, this busy work should be avoided.



This is one of the reasons I find Go to be an excellent language. Everything is egoless, everything is an `array` or a `map`. Simple. Basic. You can read more about this in the excellent article: [The Tao of Go](https://bitfieldconsulting.com/golang/tao-of-go).


*"Our life is frittered away by detail. Simplify, simplify!"* - Thoreau


## Solution

Now what, should we stop all our design processes and start building frenetically?

No, I'm not saying that we should turn off our brains today. I'm simply arguing that the things which will bring you most of the results may not be the things you are thinking about. Stop evaluating if each and every AWS service fit your problem, you will end up inventing yourself problems you don't have to justify the use of these shiny technologies (I've done that too many times in the past).



If you are going from 0 to 1, there are too many things that you don't know and are out of your control. Was the iPhone designed to have all these third-party apps that make its value today and billions of Dollars for Apple? [No!](https://appleinsider.com/articles/18/07/10/the-revolution-steve-jobs-resisted-apples-app-store-marks-10-years-of-third-party-innovation)

Whether you are working on a software project or a blog post, you will be able to update it and fix it later in time. Even this post was edited more than 10 times just the day I posted it, and way more after. So relax and ship.

**Stop trying to connect all the dots ahead of time. Embrace uncertainty and start doing**.

*"You can’t connect the dots looking forward; you can only connect them looking backwards. So you have to trust that the dots will somehow connect in your future. You have to trust in something — your gut, destiny, life, karma, whatever. This approach has never let me down, and it has made all the difference in my life."* - [Steve Jobs](https://www.youtube.com/watch?v=UF8uR6Z6KLc)

