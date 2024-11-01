import 
{
    h,
    defineComponent,
    defineAsyncComponent
  } from 'vue'

import { NAvatar, NSpin } from 'naive-ui';
import NoPhoto from  '../assets/rastr/no-ico.png';
import { image_ico, info_ico } from '../../services/svg';

const localProps = 
{
    icon: 
    {
        type: String,
        default: image_ico
    },
    size: 
    {
        type: Number,
        default: 100
    },
} as const

export default defineComponent({
props: localProps,
    setup (props) 
    {
        const icon = () =>
        {
            return h(
                NAvatar,
                {
                    style:
                    {
                        alignSelf: 'baseline',
                        width: props.size + 'px',
                        backgroundColor: "#33443300",
                    },
                    lazy: true,
                    objectFit: 'scale-down',
                    size: props.size,
                    src: props.icon
                },
            );
        }
        return {icon}
    },
    
    render ()
    {
        return h(this.icon)
    }
})