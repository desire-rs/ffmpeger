const DEFAULT_API = 'http://0.0.0.0:12306';
chrome.webRequest.onResponseStarted.addListener(async (details) => {
  const { method, url, initiator, tabId, requestId, frameId } = details;
  if (method === 'GET' && isMatch(url, '.m3u8')) {
    console.log(url, initiator);
    await chrome.storage.local.set({
      m3u8: url
    });
  }
  if (method === 'GET' && isMatch(url, '.mp4')) {
    await chrome.storage.local.set({
      mp4: url
    });
  }
}, {
  urls: ['<all_urls>']
});

function isMatch(url, target) {
  return url.indexOf(target) > -1;
}

chrome.runtime.onMessage.addListener(async (message, sender, sendResponse) => {
  let { api } = await chrome.storage.local.get('api');
  const result = await fetch(api ?? DEFAULT_API, {
    method: 'POST',
    body: JSON.stringify(message),
    headers: {
      'Content-Type': 'application/json; charset=utf-8'
    },
  });
  setTimeout(async () => {
    sendResponse(result);
  }, 200);
  return true;
});
