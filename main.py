from asyncio.tasks import ensure_future
import songbird
import asyncio


async def drive(driver):
    await driver.connect(
            token="1234",
            endpoint="1234",
            session_id="1234",
            guild_id=750862883075915826,
            channel_id=919040848237330462,
            user_id=733992836873322527
        )

async def main():

    driver =  songbird.Driver()
    await driver.make_driver()
    print(driver)
    ensure_future(drive(driver))

    while True:
        await asyncio.sleep(5)
        print("HERE")


asyncio.run(main())