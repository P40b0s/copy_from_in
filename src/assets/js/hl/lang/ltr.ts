import { LanguageFn } from "highlight.js";
const ltr: LanguageFn = function(hljs)
{
    const regex = hljs.regex;
    return {
        name: 'LTR',
        case_insensitive: true,
        unicodeRegex: false,
        contains: [
        {
            className: 'headers',
            keywords: ['[ПИСЬМО КП ПС СЗИ]', '[АДРЕСАТЫ]', '[ФАЙЛЫ]'],
        },
        ]
    };
}
export default ltr;