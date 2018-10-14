import discord
from discord.ext import commands
import asyncio
import configparser

client = commands.Bot(command_prefix=commands.when_mentioned_or('>'), description='Daddy')

listen_channels  = []
talk_to_channels = []

def config_section_map(config, section: str):
    dict1 = {}
    options = config.options(section)
    for option in options:
        try:
            dict1[option] = config.get(section, option)
            if dict1[option] == -1:
                print("skip: %s" % option)
        except:
            print("exception on %s!" % option)
            dict1[option] = None
    return dict1

bot_config = configparser.ConfigParser()
bot_config.read("res/config.ini")

auth = configparser.ConfigParser()
auth.read(config_section_map(bot_config, "Config")["auth"])

def config_add_to_listen_channels(id):
    count = int(config_section_map(bot_config, "ListenChannels")["count"])
    bot_config.set("ListenChannels", str(count), str(id))
    bot_config.set("ListenChannels", "count", str(count+1))
    with open("res/config.ini", "w") as configfile:
        bot_config.write(configfile)

def config_remove_listen_channel(id):
    count = int(config_section_map(bot_config, "ListenChannels")["count"])
    id_index = None
    for i in range(count):
        if int(config_section_map(bot_config, "ListenChannels")[str(i)]) == id:
            id_index = i
    if id_index == None:
        print("channel %s not found!" % str(id))
        return
    for i in range(id_index, count - 1):
        bot_config.set("ListenChannels", str(i), config_section_map(bot_config, "ListenChannels")[str(i+1)])
    bot_config.remove_option("ListenChannels", str(count-1))
    bot_config.set("ListenChannels", "count", str(count-1))
    with open("res/config.ini", "w") as configfile:
        bot_config.write(configfile)

def config_add_to_talk_channels(id):
    count = int(config_section_map(bot_config, "TalkChannels")["count"])
    bot_config.set("TalkChannels", str(count), str(id))
    bot_config.set("TalkChannels", "count", str(count+1))
    with open("res/config.ini", "w") as configfile:
        bot_config.write(configfile)

def config_remove_talk_channels(id):
    count = int(config_section_map(bot_config, "TalkChannels")["count"])
    id_index = None
    for i in range(count):
        if int(config_section_map(bot_config, "TalkChannels")[str(i)]) == id:
            id_index = i
    if id_index == None:
        print("channel %s not found!" % str(id))
        return
    for i in range(id_index, count - 1):
        bot_config.set("TalkChannels", str(i), config_section_map(bot_config, "TalkChannels")[str(i+1)])
    bot_config.remove_option("TalkChannels", str(count-1))
    bot_config.set("TalkChannels", "count", str(count-1))
    with open("res/config.ini", "w") as configfile:
        bot_config.write(configfile)

@client.event
async def on_ready():
    print("Logged in as:")
    print(client.user.name)
    print(client.user.id)
    print("------")

@client.event
async def on_message(message):
    for channel in listen_channels:
        if channel == message.channel.id:
            if message.content.lower().startswith("i'm"):
                await message.channel.send("Hello " + message.content[4:])
            elif message.content.lower().startswith("im"):
                await message.channel.send("Hello " + message.content[3:])
            elif message.content.lower().startswith("i am"):
                await message.channel.send("Hello " + message.content[5:])

    for channel in talk_to_channels:
        if channel == message.channel.id:
            if message.content.lower().startswith("i'm"):
                await message.channel.send("Hello " + message.content[4:])
            elif message.content.lower().startswith("im"):
                await message.channel.send("Hello " + message.content[3:])
            elif message.content.lower().startswith("i am"):
                await message.channel.send("Hello " + message.content[5:])

    
    await client.process_commands(message)

@client.command()
async def joined(ctx, member: discord.Member):
    await ctx.send('{0.name} joined in {0.joined_at}'.format(member))

@client.command(name="listen")
async def add_listen_channel(ctx, channel: discord.TextChannel):
    for chan_id in listen_channels:
        if chan_id == channel.id:
            print("already listening to channel %s!" % channel.name)
            await ctx.send("already listening to channel %s!" % channel.name)
            return
    listen_channels.append(channel.id)
    config_add_to_listen_channels(channel.id)
    print("listening to channel %s" % channel.name)
    await ctx.send("listening to channel %s" % channel.name)
    
@client.command(name="ignore")
async def remove_listen_channel(ctx, channel: discord.TextChannel):
    for chan_id in listen_channels:
        if chan_id == channel.id:
            config_remove_listen_channel(channel.id)
            listen_channels.remove(channel.id)
            print("ignoring channel %s" % channel.name)
            await ctx.send("ignoring channel %s" % channel.name)
            return
    print("not listening to channel %s!" % channel.name)
    await ctx.send("not listening to channel %s!" % channel.name)

@client.command(name="addtalk")
async def add_talk_room(ctx, channel: discord.TextChannel):
    for chan_id in talk_to_channels:
        if chan_id == channel.id:
            print("channel %s is already a talk room!" % channel.name)
            await ctx.send("channel %s is already a talk room!" % channel.name)
            return
    talk_to_channels.append(channel.id)
    print("channel %s added as talk room" % channel.name)
    await ctx.send("channel %s added as talk room" % channel.name)

@client.command(name="ignoretalk")
async def remove_talk_room(ctx, channel: discord.TextChannel):
    for chan_id in talk_to_channels:
        if chan_id == channel.id:
            talk_to_channels.remove(channel.id)
            config_remove_talk_channels(channel.id)
            print("talk channel %s is being ignored" % channel.name)
            await ctx.send("talk channel %s is being ignored" % channel.name)
            return
    print("channel %s is not a talk room!" % channel.name)
    await ctx.send("channel %s is not a talk room!" % channel.name)

def load_config():
    listen_count  = int(config_section_map(bot_config, "ListenChannels")["count"])
    talk_to_count = int(config_section_map(bot_config, "TalkChannels")["count"])

    for i in range(listen_count):
        listen_channels.append(int(config_section_map(bot_config, "ListenChannels")[str(i)]))

    for i in range(talk_to_count):
        talk_to_channels.append(int(config_section_map(bot_config, "TalkChannels")[str(i)]))

load_config()
client.run(config_section_map(auth, "DiscordAPI")["token"])