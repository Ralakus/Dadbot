import discord
from discord.ext import commands
import asyncio
import configparser

bot = commands.Bot(command_prefix='>', description='Daddy')

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
    count = config_section_map(bot_config, "ListenChannels")["count"]
    bot_config.set("ListenChannels", str(count+1), id)
    bot_config.set("ListenChannels", "count", count+1)

def config_remove_listen_channel(id):
    count = config_section_map(bot_config, "ListenChannels")["count"]
    id_index = None
    for i in range(count):
        if config_section_map(bot_config, "ListenChannels")[str(i)] == id:
            id_index = i
            bot_config.remove_option("ListenChannels", str(i))
    if id_index == None:
        print("channel %s not found!" % id)
        return
    for i in range(id_index + 1, count - 1):
        bot_config.set("ListenChannels", str(i), config_section_map(bot_config, "ListenChannels")[str(i+1)])

def config_add_to_talk_channels(id):
    count = config_section_map(bot_config, "TalkChannels")["count"]
    bot_config.set("TalkChannels", str(count+1), id)
    bot_config.set("TalkChannels", "count", count+1)

def config_remove_talk_channels(id):
    count = config_section_map(bot_config, "TalkChannels")["count"]
    id_index = None
    for i in range(count):
        if config_section_map(bot_config, "TalkChannels")[str(i)] == id:
            id_index = i
            bot_config.remove_option("TalkChannels", str(i))
    if id_index == None:
        print("channel %s not found!" % id)
        return
    for i in range(id_index + 1, count - 1):
        bot_config.set("TalkChannels", str(i), config_section_map(bot_config, "TalkChannels")[str(i+1)])

@bot.event
async def on_ready():
    print("Logged in as:")
    print(bot.user.name)
    print(bot.user.id)
    print("------")

@bot.event
async def on_message(message):
    if message.content.lower().startswith("i'm"):
        await bot.say("Hello " + message.content[4:])
    elif message.content.lower().startswith("im"):
        await bot.say("Hello " + message.content[3:])
    elif message.content.lower().startswith("i am"):
        await bot.say("Hello " + message.content[5:])
    else:
        return

@bot.command
async def joined(member: discord.Member):
    await bot.say('{0.name} joined in {0.joined_at}'.format(member))

@bot.command(name="listen")
async def add_listen_channel(channel: discord.Channel):
    print("recv")
    for chan_id in listen_channels:
        if chan_id == channel.id:
            print("already listening to channel %s!" % channel.name)
            await bot.say("already listening to channel %s!" % channel.name)
            return
    listen_channels.append(channel.id)
    config_add_to_listen_channels(channel.id)
    print("listening to channel %s" % channel.name)
    await bot.say("listening to channel %s" % channel.name)
    
@bot.command(name="ignore")
async def remove_listen_channel(channel: discord.Channel):
    for chan_id in listen_channels:
        if chan_id == channel.id:
            config_remove_listen_channel(channel.id)
            listen_channels.remove(channel.id)
            print("ignoring channel %s" % channel.name)
            await bot.say("ignoring channel %s" % channel.name)
            return
    print("not listening to channel %s!" % channel.name)
    await bot.say("not listening to channel %s!" % channel.name)

@bot.command(name="addtalk")
async def add_talk_room(channel: discord.Channel):
    for chan_id in talk_to_channels:
        if chan_id == channel.id:
            print("channel %s is already a talk room!" % channel.name)
            await bot.say("channel %s is already a talk room!" % channel.name)
            return
    talk_to_channels.append(channel.id)
    print("channel %s added as talk room" % channel.name)
    await bot.say("channel %s added as talk room" % channel.name)

@bot.command(name="ignoretalk")
async def remove_talk_room(channel: discord.Channel):
    for chan_id in talk_to_channels:
        if chan_id == channel.id:
            talk_to_channels.remove(channel.id)
            config_remove_talk_channels(channel.id)
            print("talk channel %s is being ignored" % channel.name)
            await bot.say("talk channel %s is being ignored" % channel.name)
            return
    print("channel %s is not a talk room!" % channel.name)
    await bot.say("channel %s is not a talk room!" % channel.name)

bot.run(config_section_map(auth, "DiscordAPI")["token"])