import mitt, {Emitter} from 'mitt'
import { DictionaryEditorType, EditorType, User } from '../models/user';
import { ModalEditorType } from '../modules/modals';
const emitter = mitt<Events>()
export default emitter
export {type Events, type Emitter};


type Events =
{
    /**открытие модального окна с редактором словаря тип словаря для редактирования */
    openDictionaryEditor: DictionaryEditorType;
    /**открытие модального окна с редактором карточки юзера */
    openUserCardEditor: {current_user: User, type: EditorType};
    /**открытие модального окна c редактором */
    openModalWindow: ModalEditorType;
    userUpdated: void,
};