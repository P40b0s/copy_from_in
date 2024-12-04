import 
{
    h,
    defineComponent,
    CSSProperties,
  } from 'vue'
import "./loader.scss";
import { keyframes, style, stylesheet } from 'typestyle';

const keys = keyframes({
    '100%' : {transform: "rotate(1turn)"}
})

const after_before = {
    content: "",
    gridArea: "1/1",
    margin: "10%",
    borderRadius: "50%",
    background: "inherit",
    animation: "inherit",
}
const css = stylesheet({
    spinner: 
    {
        width: '40px',
        aspectRatio: '1',
        display: "grid",
        borderRadius: "65%",
        background: "conic-gradient(#25b09b 25%,#f03355 0 50%,#514b82 0 75%,#ffa516 0)",
        animationName: keys,
        animationDuration: '2s',
        animationIterationCount: 'infinite',
        animationTimingFunction: 'linear',
        $nest:
        {
            '&::before' : after_before,
            '&::after': after_before,
        }
    },
    'spinner::after': 
    {
        margin: "25%",
        animationDuration: "3s"
    }
  });

export const Loader =  defineComponent({
    setup () 
    {
        const list = () =>
        {
            return h("div", 
            {
                class: css,
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


