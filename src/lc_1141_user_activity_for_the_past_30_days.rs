/// 1141.User Activity for the Past 30 Days
/// SQL Schema
/// Table: Activity
///
/// Colomn name     Type
///
/// user_id         int
/// session_id      int
/// activity_date   date
/// activity_type   enum
///
/// There is no primary key for this table, it may have duplicate rows.
/// The activity_type column is an ENUM of type
/// ('open_session', 'end_session', 'scroll_down', 'send_message').
/// The table shows the user activities for a social media website.
/// Note that each session belongs to exactly one user.
///
/// Write an SQL query to find the daily active user count for a period
/// of 30 days ending `2019-07-27` inclusively. A user was active on
/// someday if they made at least one activity on that day.
///
/// Return the result table in **any order**.
///
/// The query result format is in the following example.
///

/// select activity_date as day, count(distinct user_id) as active_users from Activity
/// where (activity_date >= "2019-06-27" and activity_date <= "2019-07-27")
/// group by activity_date;
#[allow(dead_code)]
fn dont_need() -> i32 {
    0
}
