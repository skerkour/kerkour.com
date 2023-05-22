```
Many template systems implement a model with a base template and other templates
that fill in the missing sections of the ba se template. They extend the base template.
This is different from the previous tec hnique, in which subtemplates were shared
among a group of different top-level template s. In this case, the top-level template is
shared.
PROBLEM
You want to have a base template and have other templates extend the base template.
The templates would have multiple sections that can be extended.
SOLUTION
Instead of thinking of a file as a template, think of sections of a file as templates. The
base file contains the shared markup and refers to other templates that haven’t yet
been defined, as shown in figure 6.2. The templates extending the base file provide
the missing subtemplates or override thos e in the base. After they’re combined, you
have a fully working template with a shared base.
DISCUSSION
The template system enables some inheritance patterns within templates. It doesn’t rep-
resent the full range of inheritance availablein other template systems, but patterns can
be applied. The following listing shows a base template for others to inherit from.
```

https://blog.questionable.services/article/approximating-html-template-inheritance/
https://gist.github.com/rakd/5073f57e5053ce443cd8de070e623d63
https://asitdhal.medium.com/golang-template-2-template-composition-and-how-to-organize-template-files-4cb40bcdf8f6
https://stackoverflow.com/questions/43816039/html-partials-in-golang
https://lets-go.alexedwards.net/sample/02.08-html-templating-and-inheritance.html
https://astaxie.gitbooks.io/build-web-application-with-golang/content/en/07.4.html
https://stackoverflow.com/questions/36617949/how-to-use-base-template-file-for-golang-html-template
