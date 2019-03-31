# 🙏🙏🙏 神棍教約 (嘲諷性練功)

```
 #####  ####### ######      #####  ####### ###  #####  #    # 
#     # #     # #     #    #     #    #     #  #     # #   #  
#       #     # #     #    #          #     #  #       #  #   
#  #### #     # #     #     #####     #     #  #       ###    
#     # #     # #     #          #    #     #  #       #  #   
#     # #     # #     #    #     #    #     #  #     # #   #  
 #####  ####### ######      #####     #    ###  #####  #    # 
```

「神棍教約」是個以太坊的智能合約教學與演練題材，「god stick」取自神棍的 google 中翻英結果。

# 💵💵💵 捐贈191版

- https://remix.ethereum.org/

```
pragma solidity 0.5.6;

contract GodStick191 {

    address public stick;
    address[] public donators;
    
    constructor () public {
        stick = msg.sender;
    }
    
    function gatherDonation() public payable {
        require(msg.value >= 0.001 ether);
        donators.push(msg.sender);
    }
}
```

# 💸💸💸 提領192版

```
pragma solidity 0.5.6;

contract GodStick192 {

    address payable public stick;
    address[] public donators;
    
    constructor () public {
        stick = msg.sender;
    }
    
    function gatherDonation() public payable {
        require(msg.value >= 0.001 ether);
        donators.push(msg.sender);
    }
    
    function transferToStick() public payable {
        require(msg.sender == stick);
        stick.transfer(address(this).balance);
    }
}
```

# 📟📟📟 空殼193版

體驗發展出 ERC20 這類標準的脈絡。

- ERC20 interface https://eips.ethereum.org/EIPS/eip-20

```
pragma solidity 0.5.6;

contract GodStick193 {

    address public stick;
    
    address[] public donators;
    
    function gatherDonation() public payable {
        // 無
    }
    
    function transferToStick() public payable {
        // 無
    }
    
}
```

# 參考資料

- Building a Crypto Church From Scratch in Fifteen Minutes. Step-by-Step Instruction https://hackernoon.com/andrii-ryzhenko-building-a-crypto-church-from-scratch-in-fifteen-minutes-step-by-step-instruction-15578f41e57d
- Jitsi Meet https://meet.jit.si/
- The 2018 Web Developer Roadmap – codeburst 

## 其他相關

- 飛天麵條神 - 維基百科 https://zh.wikipedia.org/wiki/%E9%A3%9E%E8%A1%8C%E9%9D%A2%E6%9D%A1%E6%80%AA%E7%89%A9
- How to get on Rinkeby Testnet in less than 10 minutes https://gist.github.com/cryptogoth/10a98e8078cfd69f7ca892ddbdcf26bc
- Web Payments Overview 1.0 : The methods of payment that may be used for processing the payment request. Examples include: credit card, direct debit, SEPA Credit Transfer, and Bitcoin.  https://www.w3.org/TR/webpayments-overview/
- JSR 354 provides a portable and extendible framework for handling of Money & Currency. The API models monetary amounts and currencies in a platform independent and portable way, including well defined extension points. JavaMoney http://javamoney.github.io/api.html
 https://codeburst.io/the-2018-web-developer-roadmap-826b1b806e8d

# 會後感想

## 190330 21:30 

- jitsi meet 開網頁不須註冊即可會議分享桌面很方便，人數四五人用來還可以，相較約某定點如咖啡廳地點分享方便許多，特別是需要一邊操作一邊觀看如 Remix 這類較複雜的遠端示範網頁應用的時候。
- 講合約前應該先講地址方面的東西
- 技能樹練功先以入門為先，使用瀏覽器與 Javascript 完成練習，不須安裝為主。
