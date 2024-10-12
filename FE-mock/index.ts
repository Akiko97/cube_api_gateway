import _ from 'lodash'
// 基础数据
import baseUi from './BASICE/ui.json'
import message from './BASICE/message.json'
// 用例数据
import AliceBob from './USAGES/AliceBob.json'

// 场景数据
// init场景
import AliceBobInit from './SCENES/AliceBob.init/index.json'
import AliceBobInitUi from './SCENES/AliceBob.init/ui.json'
import AliceBobInitHandles from './SCENES/AliceBob.init/handles.json'
// teardrop场景
import AliceBobTeardrop from './SCENES/AliceBob.teardrop/index.json'
import AliceBobTeardropUi from './SCENES/AliceBob.teardrop/ui.json'
import AliceBobTeardropHandles from './SCENES/AliceBob.teardrop/handles.json'
// base64
import AliceBobBase64 from './SCENES/AliceBob.base64/index.json'
import AliceBobBase64Ui from './SCENES/AliceBob.base64/ui.json'
import AliceBobBase64Handles from './SCENES/AliceBob.base64/handles.json'
// file_crypt场景
import AliceBobFileCrypt from './SCENES/AliceBob.file_crypt/index.json'
import AliceBobFileCryptUi from './SCENES/AliceBob.file_crypt/ui.json'
import AliceBobFileCryptUiHandles from './SCENES/AliceBob.file_crypt/handles.json'
// force_attack场景
import AliceBobForceAttack from './SCENES/AliceBob.force_attack/index.json'
import AliceBobForceAttackUi from './SCENES/AliceBob.force_attack/ui.json'
import AliceBobForceAttackHandles from './SCENES/AliceBob.force_attack/handles.json'


// 用例数据
const queryUsage: Record<string, any> = {
    AliceBob
}
// 场景数据
const querySence: Record<string, any> = {
    'AliceBob.init': {
        trees: AliceBobInit,
        ui: AliceBobInitUi,
        handles: AliceBobInitHandles
    },
    'AliceBob.teardrop': {
        trees: AliceBobTeardrop,
        ui: AliceBobTeardropUi,
        handles: AliceBobTeardropHandles
    },
    'AliceBob.base64': {
        trees: AliceBobBase64,
        ui: AliceBobBase64Ui,
        handles: AliceBobBase64Handles
    },
    'AliceBob.file_crypt':{
        trees: AliceBobFileCrypt,
        ui: AliceBobFileCryptUi,
        handles: AliceBobFileCryptUiHandles
     },
     'AliceBob.force_attack':{
        trees: AliceBobForceAttack,
        ui: AliceBobForceAttackUi,
        handles: AliceBobForceAttackHandles
     }
}
const _baseUi = warpBaseUiData()

function getFeApiMock(url: string, params: any) {
    if (url.includes('/getUsage')) {
        const usage_name: any = params.usage_name
        return {
            code: 0,
            msg: 'success',
            data: queryUsage[usage_name]
        }
    }

    if (url.includes('/getMessage')) {
        return {
            code: 0,
            msg: 'success',
            data: message['AliceBob'].message
        }
    }
    // 获取当前场景所有数据，包括：trees、ui、handles
    if (url.includes('/getSence')) {
        const sence_name: any = params.sence_name
        return {
            code: 0,
            msg: 'success',
            data: warpSenceData(querySence[sence_name])
        }
    }
    // 获取当前场景的handles
    if (url.includes('/getHandles')) {
        const sence_name: any = params.sence_name
        return {
            code: 0,
            msg: 'success',
            data: querySence[sence_name].handles
        }
    }
    // 获取基础的 ui 配置信息
    if (url.includes('/getBaseUi')) {
        return {
            code: 0,
            msg: 'success',
            data: _baseUi
        }
    }


    return {
        code: 1,
        msg: 'NOT FOUND',
    }
}
// 处理场景数据
function warpSenceData(sence: any) {
    const _sence = _.cloneDeep(sence)
    // 处理ui，处理ui的extend属性
    for (const key in sence.ui) {
        const item = sence.ui[key]
        _sence.ui[key] = {
            ..._.get(_baseUi, item.extend, {}),
            ...item,
        }
    }
    return _sence
}

function warpBaseUiData(): any {
    const baseUiModule = _.cloneDeep(baseUi['MODULE']) as Record<string, any>;
    for (const key in baseUiModule) {
        const item: any = baseUiModule[key];
        baseUiModule[key] = {
            ..._.get(baseUi, item.extend, {}),
            ...item
        };
    }
    const baseUiInstance = _.cloneDeep(baseUi['INSTANCE']) as Record<string, any>;
    for (const key in baseUiInstance) {
        const item: any = baseUiInstance[key];
        baseUiInstance[key] = {
            ..._.get(baseUi, item.extend, {}),
            ...item
        };
    }
    return {
        MODULE: baseUiModule,
        INSTANCE: baseUiInstance
    };
}

export default getFeApiMock;