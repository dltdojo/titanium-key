# 一個名為神棍的宗教

「一個名為神棍的宗教」是個以太坊的智能合約教學與演練題材。

# 合約一版

- https://remix.ethereum.org/

```
pragma solidity 0.5.6;

contract GodStick191 {

    address public stick;
    address[] public donators;
    
    constructor () public {
        preiest = msg.sender;
    }
    
    function gatherDonation() public payable {
        require(msg.value >= 0.001 ether);
        donators.push(msg.sender);
    }
}
```

# 合約二版

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
    
    modifier St() {
        require(msg.sender == stick);
        _;
    }
    
    function transferToStick() public St() payable {
        stick.transfer(address(this).balance);
    }
}
```

# 合約三版

Why ERC20 ? 

```
pragma solidity 0.5.6;

contract GodStick193 {

    address public stick;
    
    address[] public donators;
    
    function gatherDonation() public payable {
        // TODO
    }
    
    function transferToStick() public St() payable {
        // TODO
    }
    
    modifier St() {
       // TODO
       _;
    }
}
```

# 參考資料

- Building a Crypto Church From Scratch in Fifteen Minutes. Step-by-Step Instruction https://hackernoon.com/andrii-ryzhenko-building-a-crypto-church-from-scratch-in-fifteen-minutes-step-by-step-instruction-15578f41e57d
- Jitsi Meet https://meet.jit.si/
- 飛天麵條神 - 維基百科 https://zh.wikipedia.org/wiki/%E9%A3%9E%E8%A1%8C%E9%9D%A2%E6%9D%A1%E6%80%AA%E7%89%A9
- How to get on Rinkeby Testnet in less than 10 minutes https://gist.github.com/cryptogoth/10a98e8078cfd69f7ca892ddbdcf26bc