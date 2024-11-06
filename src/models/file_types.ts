import { archive_ico, certificate_ico, docx_ico, envelope_ico, file_ico, image_ico, journal_ico, pdf_ico, xml_ico } from "../services/svg";

export type FileType = 
{
    extension: string,
    highlighting_lang?: string,
    description: string,
    color: string,
    disabled: boolean,
    icon: string,
    type: FileTypeEnum

}
export enum FileTypeEnum
{
    /**Файл который можно прочитать как строку */
    File,
    /**Pdf файл */
    Pdf,
    /**Изображение */
    Image,
    /**Архив */
    Archive,
    /**Документ (doc, docx итд) */
    Document,
    /** Файл электронной подписи */
    Certificate,
    /**Неподдерживаемый тип файла */
    NotSupported
}
class SupportedFiles
{
    standart_color = "#b3ffba";
    inactive_color = "#919791";
    path_color = "#c8eeccba";
    error_color = "#e6a7a7c9";
    default = 
    {
        extension: "",
        description: "Выберите файл для просмотра",
        color: this.error_color,
        disabled: true,
        icon: file_ico,
        type: FileTypeEnum.NotSupported
    } as FileType
    not_supported = 
    {
        extension: "",
        description: "Неизвестный формат файла",
        color: this.error_color,
        disabled: true,
        icon: file_ico,
        type: FileTypeEnum.NotSupported
    } as FileType
    files =
    [
        {
            extension: "xml",
            highlighting_lang: "xml",
            description: "Файл с реквизитами документа, или параметрами вложения",
            color: this.standart_color,
            disabled: false,
            icon: xml_ico,
            type: FileTypeEnum.File
        },
        {
            extension: "txt",
            description: "Тестовый файл с аннотацией к документу или текстом документа",
            color: this.standart_color,
            disabled: false,
            icon: journal_ico,
            type: FileTypeEnum.File
        },
        {
            extension: "ltr",
            description: "Сопроводительный файл к транспортному пакету",
            highlighting_lang: "ini",
            color: this.standart_color,
            disabled: false,
            icon: envelope_ico,
            type: FileTypeEnum.File
        },
        {
            extension: "rc",
            description: "Файл с реквизитами документа (загружен с АРМ)",
            highlighting_lang: "xml",
            color: this.standart_color,
            disabled: false,
            icon: envelope_ico,
            type: FileTypeEnum.File
        },
        {
            extension: "zip",
            description: "Zip архив с вложением транспортного пакета",
            color: this.inactive_color,
            disabled: true,
            icon: archive_ico,
            type: FileTypeEnum.Archive
        },
        {
            extension: "pdf",
            description: "Документ в формате pdf",
            color: this.standart_color,
            disabled: false,
            icon: pdf_ico,
            type: FileTypeEnum.Pdf
        },
        {
            extension: "png",
            description: "Изображение в формате png",
            color: this.standart_color,
            disabled: false,
            icon: image_ico,
            type: FileTypeEnum.Image
        },
        {
            extension: "jpg",
            description: "Изображение в формате jpg",
            color: this.standart_color,
            disabled: false,
            icon: image_ico,
            type: FileTypeEnum.Image
        },
        {
            extension: "docx",
            description: "Текстовый файл для прсмотра в офисных приложениях",
            color: this.inactive_color,
            disabled: true,
            icon: docx_ico,
            type: FileTypeEnum.Document
        },
        {
            extension: "p7s",
            description: "Файл цифровой подписи",
            color: this.inactive_color,
            disabled: true,
            icon: certificate_ico,
            type: FileTypeEnum.Certificate
        },
    ] as FileType[]
    /**
     * 
     * @param ext Расширение файла
     */
    get_type(ext: string| undefined): FileType| undefined
    {
        return this.files.find(f=>f.extension == ext);
    }
    get_type_by_filename(filename: string| undefined): FileType| undefined
    {
        if(filename)
        {
            let ext = filename.split(".")
            let file_type = this.get_type(ext[ext.length -1])
            return file_type;
        }
        else
            return undefined;
       
    }
    sorting_order(ext: string)
    {
        switch(ext)
        {
            case "pdf":
                return 0;
            case "xml":
                return 1;
            case "png":
            case "jpg":
                return 2;
            case "ltr":
                return 3;
            case "doc":
            case "docx":
                return 4;
            case "zip":
                return 5;
            default: 
                return 6;
        }
    }
    sorting_order_by_filename(name: string)
    {
        const ext = name.split(".")
        return this.sorting_order(ext[ext.length -1])
    }
}

export const supported_files = new SupportedFiles();