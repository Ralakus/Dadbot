var Discord = require('discord.io');
var auth = require('./auth.json'); 
var cleverbot = require("cleverbot.io"),

bot = new cleverbot(auth.cbotUser,auth.cbotKey);
bot.setNick("Daddy");

bot.create(function (err, session) {
    // session is your session name, it will either be as you set it previously, or cleverbot.io will generate one for you
    
    // Woo, you initialized cleverbot.io.  Insert further code here
});

var dbot = new Discord.Client({
    token: auth.token,
    autorun: true
});

dbot.on('ready', function() {
    console.log('Logged in as %s - %s\n', dbot.username, dbot.id);
});
 
dbot.on('message', function(user, userID, channelID, message, event) {
    if(message.toLowerCase().includes("good noodle") && userID != dbot.id) {
        dbot.sendMessage({
            to: channelID,
            message: "https://youtu.be/OGQx37dwnhM"
        });
    }
    if(channelID===auth.TalkToDaddy && userID != dbot.id) {
        dbot.simulateTyping(channelID, function (err1, response1) { 
            bot.ask(message, function (err, response) {
                console.log(message);
                console.log(response);
                dbot.sendMessage({
                    to: channelID,
                    message: response
                });
            });
        });
    }
    else if(message.substring(0, 1)=='%' && userID != dbot.id) {
        dbot.simulateTyping(channelID, function (err1, response1) { 
            bot.ask(message.substring(1), function (err, response) {
                console.log(message);
                console.log(response);
                dbot.sendMessage({
                    to: channelID,
                    message: response
                });
            });
        });
    }
    else if(message.toLowerCase().startsWith("i am ")  && userID != dbot.id) {
        var sentMessage = message.substring(5);
        dbot.sendMessage({
            to: channelID,
            message: "Hello " + sentMessage + "."
        });
    }
    else if(message.toLowerCase().startsWith("i'm ")  && userID != dbot.id) {
        var sentMessage = message.substring(4);
        dbot.sendMessage({
            to: channelID,
            message: "Hello " + sentMessage + "."
        });
    }
    else if(message.toLowerCase().startsWith("im ")  && userID != dbot.id) {
        var sentMessage = message.substring(3);
        dbot.sendMessage({
            to: channelID,
            message: "Hello " + sentMessage + "."
        });
    }
    else if(message.toLowerCase().includes("retarded") && (message.toLowerCase().includes("dad bot") || message.toLowerCase().includes("dadbot")) && userID != dbot.id) {
        dbot.sendMessage({
            to: channelID,
            message: "Who are you calling retarded?"
        });
    }
    else if(message.toLowerCase().includes("shutup") && userID != dbot.id) {
        dbot.sendMessage({
            to: channelID,
            message: "No."
        });
    }
    /*if((message.toLowerCase().includes("fuck") || message.toLowerCase().includes("shit") || message.toLowerCase().includes("bitch") || message.toLowerCase().includes("cunt")) && userID != dbot.id) {
        dbot.sendMessage({
            to: channelID,
            message: "Watch your profanity; this is a Christian server."
        });
    }*/
});