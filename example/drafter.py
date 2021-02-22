#!/usr/bin/env python3

import asyncio
import json

from uuid import uuid4
from time import time_ns
from datetime import datetime, timezone
from nats.aio.client import Client as NATS

SEND_COUNT = 10


async def run(loop):
    nc = NATS()
    await nc.connect("nats://127.0.0.1:4222", loop=loop)
    for i in range(SEND_COUNT):
        utc_now = datetime.now(timezone.utc)
        current_time = time_ns()
        pending_email = {
            "id": str(uuid4()),
            "email_to": "admin@example.com",
            "email_to_name": "Tapalogi Administrator",
            "email_from": "noreply@example.com",
            "email_from_name": "Tapalogi System",
            "subject": f"Tapa Micro Mailer - Test #{current_time}",
            "body_type": "ASCII",
            "body": "Hello!! This is from example.com",
            "timestamp": utc_now.isoformat("T"),
        }
        pending_email_json = json.dumps(pending_email)
        await nc.publish("mailer.draft", pending_email_json.encode())
        print(f"SENT#{i}:\n{pending_email_json}")
    await nc.close()


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    loop.run_until_complete(run(loop))
    loop.close()
