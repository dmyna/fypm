////////////////////////////////////////////////////////////////////////////////
// fypm - The Dark Souls of productivity.
// Copyright (C) 2023-2024 Rikagaku <contact.rikagaku@gmail.com>
// Copyright (C) 2023-2024 Myna <contact@devmyna.xyz>
//
// fypm is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fypm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with fypm. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

use strum::{Display, EnumString};

#[derive(EnumString, Display, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
#[strum(serialize_all = "kebab-case")]
pub enum FypmReports {
    Waiting,
    Next,
    List,
    All,
    Blist,
    Wlist,
    Goals,
    Alarms,
    AllGoals,
    Const,
    Recurring,
    Visual,
}
#[derive(EnumString, Display, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum FypmUDAs {
    Style,
    Type,
    State,
    Mother,
    Inforelat,
    SeqCurrent,
    SeqPrevious,
    SeqNext,
    Chain,
    Wt,
    Goal,
    Alarm,
    #[strum(serialize = "effort")]
    Effort,
    #[strum(serialize = "quadrant")]
    Quadrant,
    #[strum(serialize = "estimate")]
    Estimate,
}
#[derive(EnumString, Display, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum FypmUrgency {
    // General
    #[strum(serialize = "active")]
    Active,
    #[strum(serialize = "tags")]
    Tags,
    #[strum(serialize = "project")]
    Project,
    #[strum(serialize = "annotations")]
    Annotations,
    #[strum(serialize = "scheduled")]
    Scheduled,

    // Virtual Tags
    #[strum(serialize = "OVERDUE")]
    Overdue,
    #[strum(serialize = "WAITING")]
    Waiting,
    #[strum(serialize = "TEMPLATE")]
    Template,
    #[strum(serialize = "COMPLETED")]
    Completed,
    #[strum(serialize = "DELETED")]
    Deleted,

    // WorkTime
    #[strum(serialize = "WT-Quantify!")]
    WtQuantify,
    #[strum(serialize = "WT-AllDay!")]
    WtAllDay,
    #[strum(serialize = "WT-NonSched!")]
    WtNonSched,

    // Type
    #[strum(serialize = "TYPE-Eventual")]
    TypeEventual,
    #[strum(serialize = "TYPE-Habit")]
    TypeHabit,
    #[strum(serialize = "TYPE-Objective")]
    TypeObjective,
    #[strum(serialize = "TYPE-Continuous")]
    TypeContinuous,
    #[strum(serialize = "TYPE-Check")]
    TypeCheck,
    #[strum(serialize = "TYPE-Event")]
    TypeEvent,
    #[strum(serialize = "TYPE-Goal")]
    TypeGoal,

    // Style
    #[strum(serialize = "STYLE-Apollonian")]
    StyleApollonian,
    #[strum(serialize = "STYLE-Creative")]
    StyleCreative,
    #[strum(serialize = "STYLE-Dionysian")]
    StyleDionysian,
    #[strum(serialize = "STYLE-Necessity")]
    StyleNecessity,
    #[strum(serialize = "STYLE-Idle")]
    StyleIdle,

    // Effort
    #[strum(serialize = "effort-Zero")]
    EffortZero,
    #[strum(serialize = "effort-One")]
    EffortOne,
    #[strum(serialize = "effort-Two")]
    EffortTwo,
    #[strum(serialize = "effort-Three")]
    EffortThree,
    #[strum(serialize = "effort-Four")]
    EffortFour,
    #[strum(serialize = "effort-Five")]
    EffortFive,

    // Quadrant
    #[strum(serialize = "quadrant-One")]
    QuadrantOne,
    #[strum(serialize = "quadrant-Two")]
    QuadrantTwo,
    #[strum(serialize = "quadrant-Three")]
    QuadrantThree,
    #[strum(serialize = "quadrant-Four")]
    QuadrantNone,

    // Fypm Tags
    #[strum(serialize = "SUBTASK")]
    SubTask,

    // Urgency Increment
    UrgP5,
    UrgP10,
    UrgP15,
    UrgP20,
    UrgP25,
    UrgP30,
    UrgP100,
    UrgN5,
    UrgN10,
    UrgN15,
    UrgN20,
    UrgN25,
    UrgN30,
    UrgN100,
}
