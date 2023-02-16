"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DeterministicDeployer = void 0;
const ethers_1 = require("ethers");
const utils_1 = require("ethers/lib/utils");
/**
 * wrapper class for Arachnid's deterministic deployer
 * (deterministic deployer used by 'hardhat-deployer'. generates the same addresses as "hardhat-deploy")
 */
class DeterministicDeployer {
    /**
     * return the address this code will get deployed to.
     * @param ctrCode constructor code to pass to CREATE2
     * @param salt optional salt. defaults to zero
     */
    static async getAddress(ctrCode, salt = 0) {
        return await DeterministicDeployer.instance.getDeterministicDeployAddress(ctrCode, salt);
    }
    /**
     * deploy the contract, unless already deployed
     * @param ctrCode constructor code to pass to CREATE2
     * @param salt optional salt. defaults to zero
     * @return the deployed address
     */
    static async deploy(ctrCode, salt = 0) {
        return await DeterministicDeployer.instance.deterministicDeploy(ctrCode, salt);
    }
    constructor(provider) {
        this.provider = provider;
        // from: https://github.com/Arachnid/deterministic-deployment-proxy
        this.proxyAddress = '0x4e59b44847b379578588920ca78fbf26c0b4956c';
        this.deploymentTransaction = '0xf8a58085174876e800830186a08080b853604580600e600039806000f350fe7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf31ba02222222222222222222222222222222222222222222222222222222222222222a02222222222222222222222222222222222222222222222222222222222222222';
        this.deploymentSignerAddress = '0x3fab184622dc19b6109349b94811493bf2a45362';
        this.deploymentGasPrice = 100e9;
        this.deploymentGasLimit = 100000;
    }
    async isContractDeployed(address) {
        return await this.provider.getCode(address).then(code => code.length > 2);
    }
    async isDeployerDeployed() {
        return await this.isContractDeployed(this.proxyAddress);
    }
    async deployDeployer() {
        if (await this.isContractDeployed(this.proxyAddress)) {
            return;
        }
        const bal = await this.provider.getBalance(this.deploymentSignerAddress);
        const neededBalance = ethers_1.BigNumber.from(this.deploymentGasLimit).mul(this.deploymentGasPrice);
        const signer = this.provider.getSigner();
        if (bal.lt(neededBalance)) {
            await signer.sendTransaction({
                to: this.deploymentSignerAddress,
                value: neededBalance,
                gasLimit: this.deploymentGasLimit
            });
        }
        await this.provider.send('eth_sendRawTransaction', [this.deploymentTransaction]);
        if (!await this.isContractDeployed(this.proxyAddress)) {
            throw new Error('raw TX didn\'t deploy deployer!');
        }
    }
    async getDeployTransaction(ctrCode, salt = 0) {
        await this.deployDeployer();
        const saltEncoded = (0, utils_1.hexZeroPad)((0, utils_1.hexlify)(salt), 32);
        return {
            to: this.proxyAddress,
            data: (0, utils_1.hexConcat)([
                saltEncoded,
                ctrCode
            ])
        };
    }
    async getDeterministicDeployAddress(ctrCode, salt = 0) {
        // this method works only before the contract is already deployed:
        // return await this.provider.call(await this.getDeployTransaction(ctrCode, salt))
        const saltEncoded = (0, utils_1.hexZeroPad)((0, utils_1.hexlify)(salt), 32);
        return '0x' + (0, utils_1.keccak256)((0, utils_1.hexConcat)([
            '0xff',
            this.proxyAddress,
            saltEncoded,
            (0, utils_1.keccak256)(ctrCode)
        ])).slice(-40);
    }
    async deterministicDeploy(ctrCode, salt = 0) {
        const addr = await this.getDeterministicDeployAddress(ctrCode, salt);
        if (!await this.isContractDeployed(addr)) {
            await this.provider.getSigner().sendTransaction(await this.getDeployTransaction(ctrCode, salt));
        }
        return addr;
    }
    static init(provider) {
        this._instance = new DeterministicDeployer(provider);
    }
    static get instance() {
        if (this._instance == null) {
            throw new Error('must call "DeterministicDeployer.init(ethers.provider)" first');
        }
        return this._instance;
    }
}
exports.DeterministicDeployer = DeterministicDeployer;
//# sourceMappingURL=DeterministicDeployer.js.map