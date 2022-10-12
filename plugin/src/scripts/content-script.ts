
async function sentMessage(data) {
  return new Promise((resolve, reject) => {
    chrome.runtime.sendMessage(data, response => {
      resolve(response)
    });
  })
}

async function handle(msg) {

}

chrome.runtime.onConnect.addListener((port) => {
  port.onMessage.addListener(async (msg) => {
    console.log("receive message:", msg);
    const result = await sentMessage(msg);
    setTimeout(() => {
      port.postMessage(result)
    }, 1000)
  })
});

