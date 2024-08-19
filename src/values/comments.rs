pub const TASK_CONFIG_COMMENT: &str =
"; ----------> Taskwarrior config
; In this file, you will write values exclusively for taskwarrior and its extensions.
; For UDAs, reports, urgency ando colors, you have other files, so use it only for this purpose.
;
; -> To override taskwarrior defaults, use 'overlay.fypm.ini' too!
;
; ?: See https://taskwarrior.org/docs/configuration/ to learn about the taskwarrior values.";

pub const UDA_CONFIG_COMMENT: &str =
"; ----------> User-defined attributes config
; In this file, you will write values ​​for UDAs that originate fromtamas anywhere other than fypm.
;
; -> We don't recommend to edit fypm UDAs, but if you have a good reason to do so,
;    use 'overlay.fypm.ini' for this purpose. If you are just a expert that know what are you doing,
;    how about contributing to fypm? We will appreciate it!
;
; ?: See https://taskwarrior.org/docs/udas/ to learn about the UDAs values.";

pub const REPORT_CONFIG_COMMENT: &str =
"; ----------> Reports config
; In this file, you will write values ​​for your reports. We recommend using this file to
; create your reports and edit fypm reports!
;
; -> You don't need to use 'overlay.fypm.ini' this time, because editing fypm reports
;    is encouraged and it's better to do it here.
;
; ?: See https://taskwarrior.org/docs/report/#custom-reports to learn about the reports values.";

pub const URGENCY_CONFIG_COMMENT: &str =
"; ----------> Urgency config
; In this file, you will write values for urgency.
;
; -> If you need to change the default values ​​of fypm, use this file. But you
;    should not do this in any URG_* property!
;
; ?: See https://taskwarrior.org/docs/urgency/ to learn about the urgency values.";

pub const COLORS_CONFIG_COMMENT: &str =
"; ----------> Colors config
; In this file, you will write values for colors.
;
; -> This file has a special behavior. You don't need to write values ​​for TimeWarrior, because
;    everything you write here will be converted to equivalent settings and saved for you! :)
;
; ?: See https://taskwarrior.org/docs/themes/#overriding-colors to learn about the colors values.";

pub const OVERLAY_CONFIG_COMMENT: &str =
"; ----------> Overlay config
; In this file, you will define values ​​to override any configuration.
;
; -> Do not use it to override values ​​that you have written in other configuration files,
;    as this does not make sense and will cause a lot of confusion.";