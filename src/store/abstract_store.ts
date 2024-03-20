import { reactive, readonly, watch } from "vue";

export default abstract class Store<T extends Record<string, any>> 
{
  protected state: T;
  protected abstract data(): T;
  constructor() 
  {
    const data = this.data();
    this.state = reactive(data) as T;
    // watch(() => this.state, (value) => 
    // {
    //   console.log(value);
    // }, {deep: true});
  }
  
  public getState(): T 
  {
    return readonly(this.state) as T;
  }
  
}