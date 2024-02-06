/**Базовый класс для расширения функционала хранилища */
export class BaseExtensionStore<T>
{
    protected _state: T
    constructor(state: T)
    {
        this._state = state;
    }
}