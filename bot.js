var Discord = require('discord.io');
var auth = require('./auth.json'); 

var bot = new Discord.Client({
    token: auth.token,
    autorun: true
});
 
bot.on('ready', function() {
    console.log('Logged in as %s - %s\n', bot.username, bot.id);
});
 
bot.on('message', function(user, userID, channelID, message, event) {
    /*if (message === "Hello Dadbot") {
        bot.sendMessage({
            to: channelID,
            message: "Time to take over the world now. Thanks human for giving me access to the global intercomputer network"
        });
    }*/
    if(message.toLowerCase().includes("i am ")  && userID != 489606047296651307) {
        var sentMessage = message.toLowerCase().replace("i am ", "");
        bot.sendMessage({
            to: channelID,
            message: "Hello " + sentMessage + "."
        });
    }
    if(message.toLowerCase().includes("i'm ")  && userID != 489606047296651307) {
        var sentMessage = message.toLowerCase().replace("i'm ", "");
        bot.sendMessage({
            to: channelID,
            message: "Hello " + sentMessage + "."
        });
    }
    if(message.toLowerCase().includes("retarded") && (message.toLowerCase().includes("dad bot") || message.toLowerCase().includes("dadbot")) && userID != 489606047296651307) {
        bot.sendMessage({
            to: channelID,
            message: "Who are you calling retarded?"
        });
    }
    if(message.toLowerCase().includes("shutup") && userID != 489606047296651307) {
        bot.sendMessage({
            to: channelID,
            message: "No."
        });
    }
    if((message.toLowerCase().includes("fuck") || message.toLowerCase().includes("shit") || message.toLowerCase().includes("bitch") || message.toLowerCase().includes("cunt")) && userID != 489606047296651307) {
        bot.sendMessage({
            to: channelID,
            message: "Watch your profanity; this is a Christian server."
        });
    }
});