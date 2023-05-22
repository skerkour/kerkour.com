
+++
date = 2022-08-28T06:30:00Z
title = "How to send messages to a private Telegram channel from a bot"
type = "post"
tags = ["programming", "hacking", "telegram"]
authors = ["Sylvain Kerkour"]
url = "/telegram-bot-send-messages-to-private-channel"

[extra]
lang = "en"
+++

I recently needed to send messages to a private Telegram channel from a bot to automate posting. Here is how I did to find the `chat_id` of the channel.

There are two kinds of channels: public channels with a `t.me` link, and private Telegram channels.


## Adding the bot as Administrator

In either case, you first need to add the bot as administrator: `Edit > Administrators > Add admin`.


## Bot posting a message to a public Telegram channel

Posting a message to a public channel is easy: you simply need to use the "username" of the channel. For a given channel available at `https://t.me/TestChannel123`, you can send a message using the following API endpoint: `https://api.telegram.org/botXXX:YYY/sendMessage?chat_id=@TestChannel123&text=test`

Where `XXX:YYY` is your bot token.


## Bot posting a message to a private Telegram channel

To post a message to a private channel, you first need to get the channel's "private" `chat_id`.


## Getting the `chat_id` of a private Telegram channel

For that, you first need to temporarily set your channel as public (`Edit > Channel type`) and configure a link, such as: `https://t.me/TestChannel123`.

Then, you need to send a message to this channel using the bot API: `https://api.telegram.org/botXXX:YYY/sendMessage?chat_id=@TestChannel123&text=test`

Which returns the following JSON:

```json
{
  "ok": true,
  "result": {
    "message_id": 420,
    "sender_chat": {
      "id": -1234,
      "title": "Test",
      "type": "channel"
    },
    "chat": {
      "id": -1234,
      "title": "Test",
      "type": "channel"
    },
    "date": 1641495761,
    "text": "test"
  }
}
```

Your channel's `chat_id` is the value of the `result > chat > id` field, `-123` in this case.


## Sending bot messages to a private channel

Finally, you can send a message from your bot to the channel using the following API endpoint: `https://api.telegram.org/botXXX:YYY/sendMessage?chat_id=-123&text=test`

And that's all! You can now completely automate your channel and schedule your posts using Telegram's bot API :)
