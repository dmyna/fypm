-- Your SQL goes here
DROP TABLE IF EXISTS `worktime`;
CREATE TABLE `worktimes`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`style` TEXT NOT NULL,
	`start_time` TEXT NOT NULL,
	`end_time` TEXT NOT NULL,
	`polybar_background` TEXT NOT NULL,
	`polybar_foreground` TEXT NOT NULL
);

