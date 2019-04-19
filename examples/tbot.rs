use std::env;
use std::mem;
use std::sync::Arc;

use futures::Stream;
use futures::future::{Future, IntoFuture};
use reqwest::r#async::{Client, Decoder};

use telegrambot::api::{BotApi, GetFile, SendMessage};
use telegrambot::api::rawreq::RawReq;
use telegrambot::config::Config;
use telegrambot::TelegramBot;
use telegrambot::types::{ParseMode, PhotoSize};

fn main() {
  let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
  let cfg = Config::builder(token)
    .proxy("http://127.0.0.1:1081")
    .build()
    .unwrap();

  TelegramBot::new(cfg).unwrap()
    .on_text(|(api, vtex)| {
      if let Some(reply) = &vtex.message.reply_to_message {
        reply.with_text(|vtex| {
          println!("<<<<<=====>>>> replay text message {:?}", vtex);
        })
          .with_sticker(|vtex| {
            println!("<<<<<=====>>>> replay sticker message {:?}", vtex);
          });
      }
      println!("=====> TEXT: {:?}", vtex);
    })
    .on_sticker( move |(api, sti)| {
      println!("=====> STICKER: {:?} ===> FILEID: {:?}", sti, sti.sticker.file_id);

      let fileid = sti.sticker.thumb.clone().unwrap().file_id;
      let chat = sti.message.chat.id();

      telegrambot::spawn(api.get_file(&GetFile::new(fileid))
        .and_then(move |file| {
          println!("{:?}", file);
          api.send_message(
            SendMessage::new(chat, file.unwrap().file_id)
              .parse_mode(ParseMode::Markdown))
        })
        .map(|a| println!("{:?}", a))
        .map_err(|e| eprintln!("{:?}", e)));

    })
    .on_photo(|(api, pho)| {
      println!("=====> PHOTO: {:?}", pho);
    })
    .on_document(|(api, doc)| {
      println!("=====> DOCUMENT: {:?}", doc);
    })
    .on_callback_query(|(api, cq)| {
      println!("=====> DOCUMENT: {:?}", cq);
    })
    .on_command("/start", |(api, cmd)| {
      println!("=====> COMMAND /start  {:?}", cmd);
    })
    .on_command("/list", |(api, cmd)| {
      if cmd.message.is_edited {
        return;
      }
      telegrambot::api::spawn(api.send_message(SendMessage::new(cmd.message.chat.clone().id(), "*Hello*")
        .parse_mode(ParseMode::Markdown))
        .join(api.get_me())
        .map(|(a, b)| {})
        .map_err(|e| {}));
      println!("=====> COMMAND /list  {:?}", cmd);
    })
    .start()
    .unwrap();
}
