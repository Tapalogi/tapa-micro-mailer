#!/usr/bin/env python3

import asyncio

from nats.aio.client import Client as NATS

SEND_COUNT = 10


async def run(loop):
    nc = NATS()
    await nc.connect("nats://127.0.0.1:4222", loop=loop)

    async def message_handler(msg):
        subject = msg.subject
        reply = msg.reply
        data = msg.data.decode()
        print(
            "Received a message on '{subject} {reply}': {data}".format(
                subject=subject, reply=reply, data=data
            )
        )

    sid = await nc.subscribe("mailer.sent", cb=message_handler)
    await nc.auto_unsubscribe(sid, SEND_COUNT)
    await asyncio.sleep(SEND_COUNT * 2, loop=loop)
    await nc.close()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(run(loop))
    loop.close()
