# 🙏🙏🙏 一個名為神棍的宗教

```
 #####  ####### ######      #####  ####### ###  #####  #    # 
#     # #     # #     #    #     #    #     #  #     # #   #  
#       #     # #     #    #          #     #  #       #  #   
#  #### #     # #     #     #####     #     #  #       ###    
#     # #     # #     #          #    #     #  #       #  #   
#     # #     # #     #    #     #    #     #  #     # #   #  
 #####  ####### ######      #####     #    ###  #####  #    # 
```

「一個名為神棍的宗教」是個以太坊的智能合約教學與演練題材，「god stick」取自神棍的 google 中翻英結果。

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
- 飛天麵條神 - 維基百科 https://zh.wikipedia.org/wiki/%E9%A3%9E%E8%A1%8C%E9%9D%A2%E6%9D%A1%E6%80%AA%E7%89%A9
- How to get on Rinkeby Testnet in less than 10 minutes https://gist.github.com/cryptogoth/10a98e8078cfd69f7ca892ddbdcf26bc
- Web Payments Overview 1.0 : The methods of payment that may be used for processing the payment request. Examples include: credit card, direct debit, SEPA Credit Transfer, and Bitcoin.  https://www.w3.org/TR/webpayments-overview/
- JSR 354 provides a portable and extendible framework for handling of Money & Currency. The API models monetary amounts and currencies in a platform independent and portable way, including well defined extension points. JavaMoney http://javamoney.github.io/api.html
