from asyncio import create_task, sleep

from songbird import Driver, Playable
from pincer.client import Client
from pincer.core.dispatch import GatewayDispatch
from pincer.objects import VoiceServerUpdateEvent, VoiceState

class Bot(Client):
    @Client.event
    async def on_ready(self):
        await self.send(str(GatewayDispatch(
            4,
            {
                "guild_id": 750862883075915826,
                "channel_id": 919040848237330462,
                "self_mute": False,
                "self_deaf": False
            }
        )))

        wait_for_state = create_task(self.wait_for(
            "on_voice_state_update", lambda state: state.user_id == self.bot.id)
        )
        wait_for_server = create_task(self.wait_for("on_voice_server_update"))

        state: VoiceState = await wait_for_state
        server: VoiceServerUpdateEvent = await wait_for_server

        driver = Driver()
        await driver.make_driver()
        await driver.connect(
            token=server.token,
            endpoint=server.endpoint,
            session_id=state.session_id,
            guild_id=server.guild_id,
            channel_id=state.channel_id,
            user_id=state.user_id
        )

        await driver.play(Playable.from_url("https://www.youtube.com/watch?v=HNzcxbzXwpU"))

        await sleep(1000000000000)


Bot("YOUR TOKEN HERE").run()
