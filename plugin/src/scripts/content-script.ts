interface IMessage {
  api: string;
  method: string;
  data?: Object | undefined;
}
interface IMsg {
  url: string;
}
async function sentMessage(data: IMessage) {
  return new Promise((resolve, reject) => {
    chrome.runtime.sendMessage(data, response => {
      resolve(response)
    });
  })
}

async function handle(msg: IMsg) {
  const { storage_path = '/Users/mankong/Downloads/' } = await chrome.storage.local.get('storage_path');
  const title = document.querySelector('title')?.text;
  const { url } = msg;
  const data = {
    url,
    title,
    storage_path: `${storage_path}/${title}.mp4`
  }
  const message: IMessage = {
    api: '/m3u8',
    method: 'POST',
    data,
  }
  const result = await sentMessage(message);
  return result;
}
chrome.runtime.onConnect.addListener((port) => {
  port.onMessage.addListener(async (msg: IMsg) => {
    console.log("receive message:", msg);
    const result = await handle(msg);
    setTimeout(() => {
      port.postMessage(result)
    }, 1000)
  })
});

