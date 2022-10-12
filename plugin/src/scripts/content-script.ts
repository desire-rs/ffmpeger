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
  const userAgent = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/105.0.0.0 Safari/537.36 OPR/91.0.4516.65';
  const referer = window.location.origin + '/';
  const { storagePath = '/Users/mankong/Downloads' } = await chrome.storage.local.get('storagePath');
  let title = document.title;
  const { m3u8 } = msg;
  const headers = `Host: ${new URL(m3u8).host}`
  const cmd = `ffmpeg -user_agent "${userAgent}" -referer "${referer}" -i "${m3u8}" -c copy "${title}.mp4"`
  title = title?.replace('- 成人線上直播一區 - 5278 / 5278論壇 / 我愛78論壇', '');
  title = title?.replace(' - Jable.TV | 免費高清AV在線看 | J片 AV看到飽', '');
  title = title?.replace(' - 泥巴影院 - 海外华人在线视频媒体平台，在线观看高清视频', '').replace('/', '');
  title = title?.trim();
  console.log(cmd);
  const data = {
    userAgent,
    cmd,
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

