import 
{
    h,
    defineComponent,
    CSSProperties,
  } from 'vue'
import "./loader.scss";

export const Loader =  defineComponent({
    setup () 
    {
        const list = () =>
        {
            return h("div", 
            {
                class:"spinner",
                style:
                {
                    
                } as CSSProperties
            })
        }          
        return {list}
    },
    render ()
    {
        return this.list()
    }
})