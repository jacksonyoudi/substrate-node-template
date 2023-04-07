import {ApiPromise, WsProvider, Keyring} from "@polkadot/api";

// Substrate节点的WebSocket地址
const WEB_SOCKET = 'ws://localhost:9944';

// 定义一个sleep函数，用于延迟执行
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// 连接Substrate节点
const connectSubstrate = async () => {
  const wsProvider = new WsProvider(WEB_SOCKET);
  const api = await ApiPromise.create({provider: wsProvider});
  await api.isReady;
  console.log("connection to substrate.");
  return api;
};

// 获取存活保证金
const getExistentialDeposit = async (api: ApiPromise) => {
  const deposit = await api.consts.balances.existentialDeposit.toHuman();
  console.log('const value existentialDeposit:', deposit);
  return deposit;
};

// 这是一个使用 Polkadot JS API 与 Polkadot 节点交互的 TypeScript 代码片段。

// 这个函数将一个账户绑定到 API 并打印其余额。
// 它接受一个 `api` 对象，这是来自 Polkadot JS API 的 `ApiPromise` 实例，
// 以及一个 `uri` 字符串，这是要绑定的账户的 URI。
// 它使用 `Keyring` 类创建一个新的密钥环，并从给定的 URI 添加一个账户。
// 然后它使用 `api.query.system.account` 方法获取账户信息并打印余额。
const bindAccount = async (api: ApiPromise, uri: string) => {
  const keyring = new Keyring({type: 'sr25519'}); // 创建一个新的密钥环
  const account = keyring.addFromUri(uri); // 从给定的 URI 添加一个账户
  await api.query.system.account(account.address, async (acct) => { // 获取账户信息
    const now = await api.query.timestamp.now(); // 获取当前时间戳
    const acosub = acct.data.free; // 获取账户的可用余额
    console.log(`${now} @`, uri, `balance:${acosub.toHuman()}`); // 打印余额
  });
};

// 这个函数是 `bindAccount` 的包装器，在调用 `bindAccount` 之前打印一条消息。
const printBalance = async (uri: string, name: string, api: ApiPromise) => {
  console.log(`bind@[${name}]:`); // 打印一条消息
  await bindAccount(api, uri); // 调用 `bindAccount`
};

// 这个函数使用 `api.rpc.state.getMetadata` 方法获取 Polkadot 节点的元数据并将其打印到控制台。
// 它接受一个 `api` 对象，这是来自 Polkadot JS API 的 `ApiPromise` 实例。
const getMetadata = async (api: ApiPromise) => {
  const data = await api.rpc.state.getMetadata(); // 获取元数据
  console.log('printMetadata:'); // 打印一条消息
  console.log(data); // 打印元数据
  return data;
};


// 该函数名为 `transferFormTo`，它接受四个参数：
// - `from`：要转移资金的账户的 URI。
// - `to`：接收资金的账户的 URI。
// - `api`：来自 Polkadot JS API 的 `ApiPromise` 实例。
// - `amount`：要转移的资金数量。
const transferFormTo = async (from: string, to: string, api: ApiPromise, amount: Number) => {
  const keyring = new Keyring({type: 'sr25519'}); // 创建一个新的密钥环
  const facount = keyring.addFromUri(from); // 从给定的 URI 添加一个账户
  const taccount = keyring.addFromUri(to); // 从给定的 URI 添加另一个账户
  console.log(`${from} to ${to} ${amount}`); // 打印转账信息
  await api.tx.balances.transfer(taccount.address, amount).signAndSend(facount, res => { // 创建转账交易并发送
    console.log(`Tx status:${res.status}`); // 打印交易状态
  });
};




// 这是一个 TypeScript 代码片段，用于使用 Polkadot JS API 接收 Polkadot 节点的事件。

// 该函数名为 `receiveEvent`，它接受一个参数：
// - `api`：来自 Polkadot JS API 的 `ApiPromise` 实例。
const receiveEvent = async (api: ApiPromise) => {
  api.query.system.events((events) => { // 获取 Polkadot 节点的事件
    console.log(`\nReceived ${events.length} events:`); // 打印接收到的事件数量
    events.forEach((record) => { // 对每个事件进行循环
      // 提取阶段、事件和事件类型
      const {event, phase} = record;
      if (event) {
        const types = event.typeDef;

        // 显示正在处理的内容
        console.log(` ${event.section}:${event.method}:: (phase=${phase.toString()})`);
        if (event.meta.documentation) console.log(`  ${event.meta.documentation.toString()}`);

        // 循环遍历每个参数，显示类型和数据
        event.data.forEach((data, index) => {
          console.log(`  ${types[index].type}: ${data.toString()}`);
        });
      }
    });
  });
}


const main = async () => {
  console.log('main start');
  const api = await connectSubstrate();
  await getExistentialDeposit(api);
  //var metadata = await getMetadata(api);
  //printBalance('//Alice', 'alice', api);
  //printBalance('//Bob', 'bob', api);
  //await transferFormTo('//Alice', '//Bob', api, 123 ** 12);
  receiveEvent(api);

  var loop = true;
  const stdin = process.openStdin();
  stdin.addListener("data", function (inp) {
    var action = inp.toString().trim();
    if (action == 'q') {
      console.log("quitting...");
      loop = false;
    } else {
      console.log("enter q for quit! [" + action + "]");
    }
  });

  do {
    const delay = 1000;
    await sleep(delay);
  } while (loop);
};

main().then(() => {
  console.log('success exited');
  process.exit(0);
}).catch(err => {
  console.log('error:', err);
  process.exit(1);
});
