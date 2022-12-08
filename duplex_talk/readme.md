# 使用Rust语言编写的模拟双向通信的demo

李大爷和张大爷的对话场景：
张大爷说  : "吃了没，您吶?"
李大爷说  : "刚吃。"
李大爷说  : "您这，嘛去？"
张大爷说  : "嗨！吃饱了溜溜弯儿。"
李大爷说  : "有空家里坐坐啊。"
张大爷说  : "回头去给老太太请安！"

将张大爷当作server，李大爷当作client，将以上场景重复n便，用来测试go语言编写的程序和rust语言编写的程序之间的差异。

go语言版本：fixture/refrence_go_demo.go

rust语言版本：

* 同步多线程版本：examples/no_async
* 异步多线程版本：examples/simple_async
* 异步多线程版本v2: examples/simple_async_v2

同步多线程版本使用了多线程，占用CPU资源比go版本的读很多（go版本CPU最高利用率不超过50%），多线程版本会让CPU使用率超过90%；
异步多线程版本每个读写都用task::spawn执行，设计到tokio::mutex的clone，当执行轮数超过8w后，基本就僵死不动；
异步多线程版本v2改良了以上的操作，不使用task::spawn执行读写，减少tokio::mutex的clone, 能成功执行10w轮次；