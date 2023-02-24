"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SingletonFactory__factory = void 0;
/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
const ethers_1 = require("ethers");
const _abi = [
    {
        inputs: [
            {
                internalType: "bytes",
                name: "_initCode",
                type: "bytes",
            },
            {
                internalType: "bytes32",
                name: "_salt",
                type: "bytes32",
            },
        ],
        name: "deploy",
        outputs: [
            {
                internalType: "address payable",
                name: "createdContract",
                type: "address",
            },
        ],
        stateMutability: "nonpayable",
        type: "function",
    },
];
const _bytecode = "0x608060405234801561001057600080fd5b50610173806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c80634af63f0214610030575b600080fd5b61004361003e366004610088565b61005f565b6040516001600160a01b03909116815260200160405180910390f35b6000818351602085016000f59392505050565b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561009b57600080fd5b823567ffffffffffffffff808211156100b357600080fd5b818501915085601f8301126100c757600080fd5b8135818111156100d9576100d9610072565b604051601f8201601f19908116603f0116810190838211818310171561010157610101610072565b8160405282815288602084870101111561011a57600080fd5b82602086016020830137600060209382018401529896909101359650505050505056fea2646970667358221220942f273f033772d703ba03c258b70c24fa13dd7248a7d6289b6f626bb6a62c5d64736f6c634300080f0033";
const isSuperArgs = (xs) => xs.length > 1;
class SingletonFactory__factory extends ethers_1.ContractFactory {
    constructor(...args) {
        if (isSuperArgs(args)) {
            super(...args);
        }
        else {
            super(_abi, _bytecode, args[0]);
        }
    }
    deploy(overrides) {
        return super.deploy(overrides || {});
    }
    getDeployTransaction(overrides) {
        return super.getDeployTransaction(overrides || {});
    }
    attach(address) {
        return super.attach(address);
    }
    connect(signer) {
        return super.connect(signer);
    }
    static createInterface() {
        return new ethers_1.utils.Interface(_abi);
    }
    static connect(address, signerOrProvider) {
        return new ethers_1.Contract(address, _abi, signerOrProvider);
    }
}
exports.SingletonFactory__factory = SingletonFactory__factory;
SingletonFactory__factory.bytecode = _bytecode;
SingletonFactory__factory.abi = _abi;
//# sourceMappingURL=SingletonFactory__factory.js.map