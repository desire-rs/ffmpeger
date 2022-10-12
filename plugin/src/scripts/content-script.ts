interface IMessage {
  api: string;
  method: string;
  data?: Object | undefined;
}
interface IMsg {
  m3u8: string;
}
async function sentMessage(data: IMessage) {
  return new Promise((resolve, reject) => {
    chrome.runtime.sendMessage(data, response => {
      resolve(response)
    });
  })
}

async function handle(msg: IMsg) {
  const userAgent = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36 OPR/82.0.4227.58';

  const { storagePath = '/Users/mankong/Downloads' } = await chrome.storage.local.get('storagePath');
  const title = document.querySelector('title')?.text;
  const { m3u8 } = msg;
  const data = {
    userAgent,
    url: m3u8,
    title,
    storagePath: `${storagePath}/${title}.mp4`
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

