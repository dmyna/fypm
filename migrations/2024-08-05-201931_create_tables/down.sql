-- This file should undo anything in `up.sql`
CREATE TABLE `worktime`(
	`id` TEXT PRIMARY KEY,
	`name` TEXT,
	`description` TEXT,
	`params` BINARY
);

DROP TABLE IF EXISTS `worktimes`;
