use std::{borrow::Cow, rc::Rc};
use serde::Serialize;
use serde_json::json;
use sqlx::{any, sqlite::SqliteRow, FromRow, Row};
use super::get_connection;

pub trait Id<'a>
{
    fn get_id(&'a self)-> Cow<str>;
}

pub trait Operations<'a> where Self: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin + Id<'a>
{
    fn table_name() -> &'static str;
    fn create_table() -> String;
    fn full_select() -> String;
    async fn create() ->  anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        sqlx::query(&Self::create_table())
        .execute(&mut c).await?;
        Ok(())
    }
    async fn delete(&'a self) ->  anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let sql = ["DELETE FROM ", &Self::table_name(), " WHERE id = $1"].concat();
        sqlx::query(&sql)
        .bind(self.get_id().as_ref())
        .execute(&mut c).await?;
        Ok(())
    }
    async fn update(&'a self) -> anyhow::Result<()>;
    async fn select<Q: QuerySelector<'a>>(selector: &Q) -> anyhow::Result<Vec<Self>>
    {
        let mut c = get_connection().await?;
        let query = selector.query();
        let mut res = sqlx::query_as::<_, Self>(&query.0);
        if let Some(params) = query.1
        {
            for p in params
            {
                res = res.bind(p);
            }
        };
        let r = res.fetch_all(&mut c)
        .await?;
        Ok(r)
    }

    // Теперь возник вопрос с лайфтаймами....
    // функция трейта:
    // ```rust
    // async fn execute<Q: QuerySelector<'a>>(selector: &Q) -> anyhow::Result<()>
    // ```
    // ```rust
    // pub trait QuerySelector<'a> where Self: Sized
    // ```
    // имплементирую трейт, создаю экземпляр и передаю как параметр в функцию execute
    // ```rust
    // impl<'a> QuerySelector<'a> for Selector<'a>
    // ```
    // execute одна из функций трейта:
    // ```rust
    // pub trait Operations<'a, T: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin + Id<'a>>
    // ```
    // если реализую еще одну функцию прямо в трейте и пишу ```Self::execute(&sql).await?;```
    // то получаю какую-то не особо понятную ошибку с временем жизни, понятно что в фунцию передается трейт не с тем временем жизни что нужно, а что нужно непонятно
    // если execute вызывать из реализации трейта то все в поряке...
    // error: implementation of `operations::Operations` is not general enough
    // = note: `operations::QuerySelector<'1>` would have to be implemented for the type `operations::Selector<'0>`, for any two lifetimes `'0` and `'1`...
    // = note: ...but `operations::QuerySelector<'2>` is actually implemented for the type `operations::Selector<'2>`, for some specific lifetime `'2`

    async fn execute<Q: QuerySelector<'a>>(selector: &Q) -> anyhow::Result<()>
    {
        let mut c = get_connection().await?;
        let query = selector.query();
        let mut exe = sqlx::query(&query.0);
        if let Some(params) = query.1
        {
            for p in params
            {
                exe = exe.bind(p);
            }
        };
        exe.execute(&mut c).await?;
        Ok(())
    }
    async fn select_special_type<Q: QuerySelector<'a>,
    O: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin>(selector: &Q) -> anyhow::Result<Vec<O>>
    {
        let mut c = get_connection().await?;
        let query = selector.query();
        let mut res = sqlx::query_as::<_, O>(&query.0);
        if let Some(params) = query.1
        {
            for p in params
            {
                res = res.bind(p);
            }
        };
        let r = res.fetch_all(&mut c)
        .await?;
        Ok(r)
    }
    async fn get_one<Q: QuerySelector<'a>,
    R: for<'r> sqlx::FromRow<'r, SqliteRow> + Send + Unpin>(selector: &Q) -> anyhow::Result<R>
    {
        let mut c = get_connection().await?;
        let query = selector.query();
        let mut res = sqlx::query_as::<_, R>(&query.0);
        if let Some(params) = query.1
        {
            for p in params
            {
                res = res.bind(p);
            }
        };
        let r = res.fetch_one(&mut c)
        .await?;
        Ok(r)
    }
    async fn add_or_replace(&'a self) -> anyhow::Result<()>;
    async fn add_or_ignore(&'a self) -> anyhow::Result<()>;
    ///удаляет все id которых нет в списке
    ///WHERE id NOT IN ('...', '...')
    async fn delete_many_exclude_ids(ids: Vec<String>, user_id: Option<&'a str>) -> anyhow::Result<()>
    {
       
        let del = ["DELETE FROM ", Self::table_name()].concat();
        let mut sql = Selector::new(&del)
        .where_not_in(&ids);
        if let Some(uid) = user_id 
        {
            sql = sql.and("user_id", "=", &uid);
        }
        let mut c = get_connection().await?;
        let query = sql.query();
        let exe = sqlx::query(&query.0);
        exe.execute(&mut c).await?;
        //FIXME Self::execute глючит по лайфтаймам незнаю пока как иправить...
        Ok(())
    }
    
    
}

#[derive(Debug, Clone, FromRow)]
pub struct CountRequest
{
    pub count: u32
}
#[derive(Debug, Clone)]
pub struct IdSelector(pub String);
impl FromRow<'_, SqliteRow> for IdSelector
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        let id: String = row.try_get("id")?;
        Ok(Self(id))
    }
}


#[derive(Debug, Clone)]
pub enum SortingOrder<'a>
{
    Asc(&'a str),
    Desc(&'a str)
}
#[derive(Debug, Clone)]
pub struct Selector<'a>
{
    query: String,
    where_params: Option<Vec<(String, String)>>,
    and_params: Option<Vec<(String, String, String)>>,
    where_by_id: Option<String>,
    sorting_order: Option<SortingOrder<'a>>,
    limit: Option<&'a u32>,
    offset: Option<&'a u32>,
}
///Основные параметры нашей таблицы<br>
/// к сожалению константу нельзя соединять с константой поэтому имя заблицы придется дублировать во всех константах
// pub trait SelectQuery
// {
//     ///Полное тело запроса SELECT 1, 2, 3 from table
//     const SELECT_BODY: &'static str;
//     ///имя таблицы, чтобы можно было его использовать при запросах
//     const TABLE_NAME: &'static str;
//     ///Код создания таблицы
//     const CREATE_TABLE: &'static str;
// }

pub fn from_json<V: for<'a> serde::de::Deserialize<'a>, S : AsRef<str>>(row: &SqliteRow, row_name: S) -> Option<V>
{
    let sender_info: Option<String> = row.try_get(row_name.as_ref()).ok()?;
    if let Some(r) = sender_info
    {
        Some(serde_json::from_str::<V>(&r).unwrap())
    }
    else
    {
        None
    }
}

pub trait QuerySelector<'a> where Self: Sized
{
    ///Создаем новый экземпляр селектора и даем ему полный селект нашей таблицы (SELECT_BODY)
    fn new<S: AsRef<str> + ToString>(select: S) -> Self;
    fn new_concat<I: IntoIterator<Item = S>, S: AsRef<str>>(select: I) -> Self;
    ///Делаем запрос на основе селектора
    fn query(&self) -> (String, Option<Vec<String>>);
    fn where_not_in(self, ids: &[String]) -> Self;
    fn where_in(self, ids: &[String]) -> Self;
    ///добавляем параметр и значение параметра для выборки WITH
    fn add_param<T: ToString>(self, param: &str, value: &T) -> Self;
    ///динамическое добавление параметров в запрос
    fn add_params(self, params: Option<Vec<(&str, &str)>>) -> Self;
    fn and<T: ToString>(self, param: &str, operator: &str, value: &T) -> Self;
    fn add_raw_query(self, raw: &str) -> Self;
    fn limit(self, raw: &'a u32) -> Self;
    fn offset(self, raw: &'a u32) -> Self;
    ///добавляем параметр и объект для выборки WITH в jsone
    /// param = requisites->'mj'->'number' value = 72097
    fn add_json_param<T: ToString>(self, param: &str, value: &T) -> Self;
    ///Сортировка по возрастанию или убыванию (необходимо указать по какому столбцу будет проводиться сортировка)
    fn sort(self, sotring_order: SortingOrder<'a>) -> Self;
}
impl<'a> QuerySelector<'a> for Selector<'a>
{
    fn new<S: AsRef<str> + ToString>(select: S) -> Self
    {
        Self
        {
            query: select.to_string(),
            where_params: None,
            and_params: None,
            sorting_order: None,
            limit: None,
            offset: None,
            where_by_id: None
        }
    }
    fn new_concat<I: IntoIterator<Item = S>, S: AsRef<str>>(select: I) -> Self
    {
        let c = select.into_iter().map(|m| String::from(m.as_ref())).collect::<String>();
        Self
        {
            query: c,
            where_params: None,
            and_params: None,
            sorting_order: None,
            limit: None,
            offset: None,
            where_by_id: None
        }
    }
    fn where_in(mut self, ids: &[String]) -> Self
    {
        let ids : String = ids.into_iter().map(|m| ["\"", m, "\""].concat()).collect::<Vec<String>>().join(",");
        let id_in = [" WHERE id IN ", "(", &ids, ")"].concat();
        self.where_by_id = Some(id_in);
        self
    }
    fn where_not_in(mut self, ids: &[String]) -> Self
    {
        let ids : String = ids.into_iter().map(|m| ["\"", m, "\""].concat()).collect::<Vec<String>>().join(",");
        let id_in = [" WHERE id NOT IN ", "(", &ids, ")"].concat();
        self.where_by_id = Some(id_in);
        self
    }
    fn query(&self) -> (String, Option<Vec<String>>)
    {
        let mut body : String = self.query.clone();
        let mut values: Option<Vec<String>> = None;
        let mut contains_where = false;
        if let Some(where_p) = self.where_params.as_ref()
        {
            body.push_str(" WHERE ");
            values = Some(vec![]);
            for (i, (par, val)) in where_p.iter().enumerate()
            {
                let delimitter = if where_p.len() > 1 && (i+1) < where_p.len()
                {
                    " AND "
                }
                else
                {
                    ""
                };
                let w = [par, " = $", &(i+1).to_string(), delimitter].concat();
                values.as_mut().unwrap().push(val.to_owned());
                body.push_str(&w);
            }
            contains_where = true;
        }
        else if let Some(win) = self.where_by_id.as_ref()
        {
            body.push_str(win);
            contains_where = true;
        };
        if contains_where == true
        {
            if let Some(and) = self.and_params.as_ref()
            {
                for (name, operator, val) in and.iter()
                {
                    let w = [" AND ", name, operator, "\"", val, "\""].concat();
                    body.push_str(&w);
                }
            }
        };
        if let Some(order) = self.sorting_order.as_ref()
        {   
            let ord = match order
            {
                SortingOrder::Asc(p) => [" ORDER BY ", p, " ASC"].concat(),
                SortingOrder::Desc(p) => [" ORDER BY ", p, " DESC"].concat()
            };
            body.push_str(&ord);
        }
        if let Some(lim) = self.limit
        {   
            let sql = [" LIMIT ", lim.to_string().as_str()].concat();
            body.push_str(&sql);
        }
        if let Some(off) = self.offset
        {   
            let sql = [" OFFSET ", off.to_string().as_str()].concat();
            body.push_str(&sql);
        }
        (body,values)
    }

    fn add_param<T: ToString>(mut self, param: &str, value: &T) -> Self
    {
        if self.where_params.is_none()
        {
            self.where_params = Some(vec![]);
        }
        self.where_params.as_mut().unwrap().push((param.to_owned(), value.to_string()));
        self
    }
    fn add_params(mut self, params: Option<Vec<(&str, &str)>>) -> Self
    {
        if params.is_none()
        {
            return self;
        }
        if self.where_params.is_none()
        {
            self.where_params = Some(vec![]);
        }
        for (p, v) in params.unwrap().into_iter()
        {
            self.where_params.as_mut().unwrap().push((p.to_owned(), v.to_owned()));
        }
        self
    }
    fn and<T: ToString>(mut self, param: &str, operator: &str, value: &T) -> Self
    {
        if self.and_params.is_none()
        {
            self.and_params = Some(vec![]);
        }
        self.and_params.as_mut().unwrap().push((param.to_owned(), operator.to_owned(), value.to_string()));
        self
    }
    fn add_raw_query(mut self, raw: &str) -> Self
    {
        self.query.push_str(raw);
        self
    }
    fn add_json_param<T: ToString>(mut self, param: &str, value: &T) -> Self
    {
        if self.where_params.is_none()
        {
            self.where_params = Some(vec![]);
        }
        //"requisites->'mj'->'number' = '\"72097\"'"
        self.where_params.as_mut().unwrap().push((param.to_owned(),  ["'","\"", &value.to_string(), "\"", "'"].concat()));
        self
    }
    fn sort(mut self, sotring_order: SortingOrder<'a>) -> Self
    {
        self.sorting_order = Some(sotring_order);
        self
    }
    fn limit(mut self, limit: &'a u32) -> Self
    {
        self.limit = Some(limit);
        self
    }
    fn offset(mut self, offset: &'a u32) -> Self
    {
        self.offset = Some(offset);
        self
    }

}


#[cfg(test)]
mod tests
{

    use super::{QuerySelector, SortingOrder};


    #[test]
    fn test_query_generic()
    {
        let str_1 = "123";
        let str_2 = String::from("1строка");
        let num = 8u32;
        let b = true;
        let q = super::Selector::new("SELECT one, two FROM tests")
        .add_param("one", &str_1)
        .add_param("two", &str_2)
        .add_param("num", &num)
        .add_param("bool", &b)
        .add_json_param("requisites->'mj'->'number'", &72097)
        .sort(SortingOrder::Asc("two"))
        .query();
    
        assert_eq!(q.0, "SELECT one, two FROM tests WHERE one = $1 AND two = $2 AND num = $3 AND bool = $4 AND requisites->'mj'->'number' = $5 ORDER BY two ASC");
        for v in q.1.as_ref().unwrap()
        {
            println!("{:?}", v)
        }
    }

    #[test]
    fn test_query_generic_2()
    {
        let ids = vec!["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()];
        let q = super::Selector::new("SELECT one, two FROM tests")
        .where_in(&ids)
        .and("user_id", "=", &"12345")
        .sort(SortingOrder::Asc("two"))
        .query();
        assert_eq!(q.0, "SELECT one, two FROM tests WHERE id IN (\"1\",\"2\",\"3\",\"4\") AND user_id=\"12345\" ORDER BY two ASC");
    }
    #[test]
    fn test_sql_query()
    {
        let user_id = Some("123".to_owned());
        let ids = vec!["1".to_owned(), "2".to_owned(), "3".to_owned(), "4".to_owned()];
        let del = ["DELETE FROM ", "111"].concat();
        let mut sql = super::Selector::new(&del)
        .where_not_in(&ids);
        if let Some(uid) = user_id 
        {
            sql = sql.and("user_id", "=", &uid);
        }
        assert_eq!(sql.query().0, "DELETE FROM 111 WHERE id NOT IN (\"1\",\"2\",\"3\",\"4\") AND user_id=\"123\"");
    }
}