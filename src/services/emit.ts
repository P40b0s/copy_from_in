import mitt, {Emitter} from 'mitt'
const emitter = mitt<Events>()
export default emitter
export {type Events, type Emitter};


type Events =
{
    userUpdated: void,
};