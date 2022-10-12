chrome.webRequest.onResponseStarted.addListener(async (details) => {
  const { method, url, initiator, tabId, requestId, frameId } = details;
  console.log(details);
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

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log(message, sender);
  setTimeout(async () => {
    sendResponse(message);
  }, 200);
  return true;
});
