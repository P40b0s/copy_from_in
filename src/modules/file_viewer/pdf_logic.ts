import { Ref, ref } from "vue";
import { Status } from "naive-ui/es/progress/src/interface";
import {type File} from '../../models/types';

export const pdf_logic = (is_open: Ref<boolean>, in_progress: Ref<boolean>) =>
{
    const current_image = ref<string>();
    const current_page = ref(1);
    const current_file = ref<File>();
    const pages = ref(1);
    const errors = ref<string[]>([]);
    //let is_thumb = false;

    // const render_status = (status: PdfRenderStatus) =>
    // {
    //     errors.value = status.errors;
    //     percentage.value = status.percentage;
    //     in_progress.value = true;
    //     pages.value = status.pages;
    // }
    const response_pdf_pages = async (pages: string[]) => 
    {
        const img = 'data:image/png;base64,' +  pages[0];
        current_image.value = img;
        in_progress.value = false;
    }
    const select_pdf = (path: string) =>
    {
        if (current_pdf_path.value != path)
        {
            current_pdf_path.value = path;
            current_page.value = 1;
            change_page(1);
            percentage.value = 0;
        }
        is_open.value = true;
    }
    
    const change_page = (pagenum: number) =>
    {
        in_progress.value = true;
        render_page(pagenum);
        percentage.value = 0;
    }
    //true, "15933154/text0000000000.pdf"
    const render_page = (page_num: number) =>
    {
        if(current_pdf_path.value == "")
        {
            console.error("Для рендера не выбран pdf файл!");
            return;
        }
        //const render_page_command = create_pdf_command(false, current_pdf_path.value, page_num);
        //ws.send_message(render_page_command);
    }

    const on_wheel = (e: WheelEvent) =>
    {
        if(!in_progress.value)
        {
           
            if (e.deltaY > 0)
            {
                const target_page = current_page.value + 1;
                if (target_page <= pages.value)
                {
                    change_page(target_page)
                    current_page.value = target_page;
                }
            }
            else
            {
                const target_page = current_page.value - 1;
                if (target_page > 0)
                {
                    change_page(target_page)
                    current_page.value = target_page;
                }
            }
            e.stopPropagation();
        }
    }

    return {
    render_page,
    change_page,
    select_pdf,
    response_pdf_pages,
    current_image,
    current_page,
    in_progress,
    pages,
    errors,
    on_wheel
    }
}