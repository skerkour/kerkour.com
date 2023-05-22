+++
date = 2022-03-30T02:00:00Z
title = "Last week's newsletter incident postmortem"
type = "post"
tags = ["programming", "go"]
authors = ["Sylvain Kerkour"]
url = "/2022-03-23-newsletter-incident-postmortem"

[extra]
lang = "en"

comment ="""
"""
+++


Last week, I moved [my weekly newsletter](https://kerkour.com/subscribe) from Mailchimp to my own program, and the worse happened.

Like every week, I write the newsletter from Monday to Wednesday and schedule it for Wednesday afternoon. After editing it and having fixed typos, I scheduled it for `2022-03-23 12:15:00 UTC` and started coding on something else.

During my break, I checked my emails and found it weird that I didn't receive the newsletter, while my dashboard displayed a nice `Sent` status flag.

Then I started to receive messages from subscribers telling me that something was completely wrong with the template of the email or that it made their email client crash 😱

I inspected the log of my server, and most of the emails were rejected by my email provider.

In pseudo-code, the incriminated piece of code was as follow:
```go
func SendNewsletter() {
    // ..

    var emailBodyBuffer bytes.Buffer

    for _, recipient := range recipients {
        unsubscribeLink, err := service.members.GetMemberUnsubscribeLink(ctx, site.PrimaryDomain, recipient.ID)
        if err != nil {
          log.Error(err)
          err = nil
          continue
        }

        emailData := sites.BroadcastEmailData{
          Title:           broadcast.Title,
          Content:         template.HTML(broadcast.BodyHTML),
          UnsubscribeLink: template.URL(unsubscribeLink),
        }
        err = service.broadcastEmailTemplate.Execute(&emailBodyBuffer, emailData)
        if err != nil {
          log.Error(err)
          err = nil
          continue
        }

        err = service.mailer.SendBroadcast(ctx, mailServerToken, message)
        if err != nil {
          log.Error(err)
          err = nil
          continue
        }
    }
}
```


All the evil lies in the `emailBodyBuffer` variable that is reused for all the recipients.

As [`io.Writer`](https://pkg.go.dev/io#Writer) is the Go equivalent of a stream, the [`Template.Execute`](https://pkg.go.dev/html/template#Template.Execute) method only appends data, no `seek(0)` is performed.

Thus,

* recipient 1 received: `[email for recipient 1]`
* recipient 2 received: `[email for recipient 1][email for recipient 2]`
* recipient 3 received: `[email for recipient 1][email for recipient 2][email for recipient 3]`
* ...

And after a few dozen / hundreds recipients, the mailing server stopped accepting emails because the email body was too large.

But worse, it means that recipient 3 received the unsubscribe links for both recipients 1 and 2.


## Remediation

Fortunately, the unsubscribe links can be revoked server-side, and I did it as soon as I found the bug.

Then I fixed the code by moving the `emailBodyBuffer` variable inside the loop.

```go
for _, recipient := range recipients {
    var emailBodyBuffer bytes.Buffer
    // ...
}
```

And deployed the service again.

The Lesson? More tests are needed :)
