import {  Ref } from 'vue';

const sleepNow = (delay: number) => new Promise((resolve) => setTimeout(resolve, delay))
const timer  =  (delay: number) =>  setTimeout(()  => { }, delay);

let tm : NodeJS.Timeout = setTimeout(()  => { }, 200); // eslint-disable-line

const groupByArray = (xs: any, key: any) => 
{ 
  return xs.reduce(function (rv: any, x: any) 
  { 
    const v: any = key instanceof Function ? key(x) : x[key];
    const el = rv.find((r:any) => r && r.key === v);
    if (el) 
    {
      el.values.push(x);
    } 
    else 
    { 
      rv.push({ key: v, values: [x] });
    } 
    return rv; 
  }, []); 
}
const group = <T, K extends keyof any>(list: T[], getKey: (item: T) => K) =>
list.reduce((previous, currentItem) => 
{
    const group = getKey(currentItem);
    if (!previous[group]) previous[group] = [];
    previous[group].push(currentItem);
    return previous;
}, {} as Record<K, T[]>);

const groupBy = (list: any, keyGetter: any) =>
{
    const map = new Map();
    list.forEach((item: any) => {
         const key = keyGetter(item);
         const collection = map.get(key);
         if (!collection) {
             map.set(key, [item]);
         } else {
             collection.push(item);
         }
    });
    return map;
}

/**Рекурсивное получение предка итема по id его потомка*/
// const GetParentItemCallback = (loopData: IItem[], id: string, callback: any) => 
//     {
//       loopData.forEach((item, index, arr) => 
//       {
//         if (item.Id == id) 
//         {
//           return callback(item, index, arr);
//         }
//         if (item.Items && item.Items.length > 0) 
//         {
//           return GetParentItemCallback(item.Items, id, callback);
//         }
//       });
//     }

//     const GetParentItem = (loopData: IItem[], id: string) => 
//     {
//       let i :IItem | undefined;
//       loopData?.forEach((item, index, arr) => 
//       {
//         if (item.Id == id) 
//         {
//           i = item;
//           //return item;
//         }
//         if (item.Items && item.Items.length > 0) 
//         {
//           i = GetParentItem(item.Items, id);
//           //return GetParentItem(item.Items, id);
//         }
//       });
//       return i;
//     }

//     // const GetItemByParentIndexLoop = (loopData: IItem[], par: number, callback: any) => 
//     // {
//     //   if(loopData)
//     //   {
//     //     loopData.forEach((item, index, arr) => 
//     //     {
//     //         if (item.ElementIndex == par) 
//     //         {
//     //         return callback(item, index, arr);
//     //         }
//     //         if (item.Items && item.Items.length > 0) 
//     //         {
//     //         return GetItemByParentIndexLoop(item.Items, par, callback);
//     //         }
//     //     });
//     //    }
//     // };

//     // const GetParentHeader = (loopData: IHeader[], id: Guid, callback: any) => 
//     // {
//     //   if(loopData)
//     //   {
//     //     loopData.forEach((header, index, arr) => 
//     //     {
//     //       if(header.Items && header.Items.length > 0)
//     //       {
//     //         if (header.Items.some(s=>s.Id == id)) 
//     //         {
//     //           return callback(header, index, arr);
//     //         }
//     //          return GetParentItem(header.Items, id, callback);
//     //       }
//     //       if(header.Indents && header.Indents.length > 0)
//     //       {
//     //         if (header.Indents.some(s=>s.Id == id)) 
//     //         {
//     //           return callback(header, index, arr);
//     //         }
//     //       }
//     //     });
//     //    }
//     // };

//     /**Получаем заголовок абзаца или итема */
//     function GetParentHeader<I extends IIndent | IItem>(loopData: IHeader[], child : I) : IHeader | undefined
//     {
//       if(loopData)
//       {
//         if(child.nodeType == NodeType.Абзац)
//         {
//           loopData.forEach((header, index, arr) => 
//           {
//             if(header.Indents && header.Indents.length > 0)
//             {
//               if (header.Indents.some(s=>s.Id == child.Id)) 
//               {
//                 return header;
//               }
//             }
//           });
//         }
//         else
//         {
//           loopData.forEach((header, index, arr) => 
//           {
//             if(header.Items && header.Items.length > 0)
//               {
//                 if(header.Items.some(s=>s.Indents.some(i=>i.Id == child.Id)))
//                 {
//                   return header;
//                 }
//                 if (header.Items.some(s=>s.Id == child.Id)) 
//                 {
//                   return header;
//                 }
//                 return GetParentItem(header.Items, child.Id);
//               }
//           });
//         }
//        }
//        else return undefined;
//     }


  function isError (e: unknown)  : [string, string]
    {
      if(e instanceof Error)
      {
        console.log(e.name);
        return [e.name, e.message];
      }
      return ["", ""];
    }

    function deep_clone<T>(obj: T): T
    {
      return JSON.parse(JSON.stringify(obj)) as T;
    }


    const get_date_time = (format: DateFormat, date? : string| Date): string =>
    {
      if (date)
      {
        const dt = date instanceof Date ? date : new Date(date);
        const month = dt.getMonth().toString().length == 1 ? '0' + (dt.getMonth() + 1) : dt.getMonth() + 1;
        const day = dt.getDate().toString().length == 1 ? '0' + (dt.getDate()) : dt.getDate();
        const year = dt.getFullYear();
        const hours = dt.getHours();
        const minutes =  dt.getMinutes().toString().length == 1 ? '0' + (dt.getMinutes()) : dt.getMinutes();
        switch (format)
        {
          case DateFormat.DotDate:
          {
            return day + "." + month + "." + year;
          }
          case DateFormat.DashDate:
          {
            return day + "-" + month + "-" + year;
          }
          case DateFormat.DotDateTime:
          {
            return day + "." + month + "." + year + " " + hours + ":" + minutes;
          }
          case DateFormat.DashDateTime:
          {
            return day + "-" + month + "-" + year + " " + hours + ":" + minutes;
          }
          case DateFormat.IsoDateTime:
          {
            return year + "-" + month + "-" + day + "T" + hours + ":" + minutes;
          }
          case DateFormat.WordDate:
          {
            return day + " " + word_date(month) + " " + year + " года";
          }
        }
      }
      else
      {
        return "";
      }
    }

    const word_date = (month: string | number) : string =>
    {
      return dates_map.get(month) as string;
    }
    const dates_map = new Map<string|number, string>([
      ["01", "января"],
      ["02", "февраля"],
      ["03", "марта"],
      ["04", "апреля"],
      ["05", "мая"],
      ["06", "июня"],
      ["07", "июля"],
      ["08", "августа"],
      ["09", "сентября"],
      ["10", "октября"],
      ["11", "ноября"],
      ["12", "декабря"],
    ]);

    enum DateFormat
    {
      WordDate,
      DotDate,
      DashDate,
      DashDateTime,
      IsoDateTime,
      DotDateTime

    }


export {sleepNow, timer, isError, groupBy, get_date_time, DateFormat, deep_clone, group}