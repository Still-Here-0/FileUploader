-- DROP SCHEMA uploader;

CREATE SCHEMA uploader;

-- uploader.BOARD definition

-- Drop table

-- DROP TABLE uploader.BOARD;

CREATE TABLE uploader.BOARD (
	pk int IDENTITY(1,1) NOT NULL,
	Name varchar(50) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	Active bit DEFAULT 1 NOT NULL,
	CONSTRAINT Board_PK PRIMARY KEY (pk),
	CONSTRAINT Board_UNIQUE UNIQUE (Name)
);

-- Extended properties

EXEC sys.sp_addextendedproperty @name=N'MS_Description', @value=N'Super users have full access to all of the boards with all permitions
Super users are meant to the from the digital team', @level0type=N'Schema', @level0name=N'uploader', @level1type=N'Table', @level1name=N'BOARD';


-- uploader.COLUMN_TYPE definition

-- Drop table

-- DROP TABLE uploader.COLUMN_TYPE;

CREATE TABLE uploader.COLUMN_TYPE (
	pk int IDENTITY(1,1) NOT NULL,
	SqlType varchar(30) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	ViewType varchar(50) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	CONSTRAINT COLUMN_TYPE_PK PRIMARY KEY (pk)
);


-- uploader.HIST_GROUP definition

-- Drop table

-- DROP TABLE uploader.HIST_GROUP;

CREATE TABLE uploader.HIST_GROUP (
	Group_fk int NOT NULL,
	Name varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI DEFAULT NULL NULL,
	Active bit DEFAULT NULL NULL,
	AddWorker bit DEFAULT NULL NULL,
	EditWorker bit DEFAULT NULL NULL,
	AddProfile bit DEFAULT NULL NULL,
	RemoveProfile bit DEFAULT NULL NULL,
	AddGroup bit DEFAULT NULL NULL,
	RemoveGroup bit DEFAULT NULL NULL,
	EditGroup bit DEFAULT NULL NULL,
	EditProfileGroups bit DEFAULT NULL NULL,
	EditedBy_fk int NOT NULL,
	EditedAt datetime DEFAULT getdate() NOT NULL,
	EditAction varchar(30) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	ImpersonateUsers bit DEFAULT NULL NULL
);


-- uploader.HIST_SHEET definition

-- Drop table

-- DROP TABLE uploader.HIST_SHEET;

CREATE TABLE uploader.HIST_SHEET (
	Sheet_fk int NOT NULL,
	Description varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NULL,
	TableName varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NULL,
	EditBy_fk int NOT NULL,
	Active bit NULL,
	DaysToRefresh int NULL,
	Model varbinary(MAX) NULL,
	EditAction varchar(30) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	EditedAt datetime DEFAULT getdate() NOT NULL
);

-- uploader.HIST_SHEET_META_DATA definition

-- Drop table

-- DROP TABLE uploader.HIST_SHEET_META_DATA;

CREATE TABLE uploader.HIST_SHEET_META_DATA (
	SheetMetaData_fk int NOT NULL,
	Sheet_fk int NULL,
	ColumnName varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NULL,
	ColumnType_fk int NULL,
	Optional bit NULL,
	RegexConstraint varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NULL,
	EditedBy_fk varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	EditAction varchar(30) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	EditedAt datetime DEFAULT getdate() NOT NULL
);


-- uploader.HIST_UPLOADER_PERMISSION definition

-- Drop table

-- DROP TABLE uploader.HIST_UPLOADER_PERMISSION;

CREATE TABLE uploader.HIST_UPLOADER_PERMISSION (
	Group_fk int NOT NULL,
	Sheet_fk int NOT NULL,
	CanViewHist bit NULL,
	CanUpload int NULL,
	EditedBy_fk int NOT NULL,
	ActionHist varchar(30) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	EditedAt datetime DEFAULT getdate() NOT NULL
);


-- uploader.WORKER definition

-- Drop table

-- DROP TABLE uploader.WORKER;

CREATE TABLE uploader.WORKER (
	pk int IDENTITY(1,1) NOT NULL,
	Name varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	LindeId char(6) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	Email varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	CONSTRAINT WORKER_UNIQUE UNIQUE (LindeId),
	CONSTRAINT Worker_PK PRIMARY KEY (pk)
);


-- uploader.PROFILE definition

-- Drop table

-- DROP TABLE uploader.PROFILE;

CREATE TABLE uploader.PROFILE (
	pk int IDENTITY(1,1) NOT NULL,
	Active bit DEFAULT 1 NOT NULL,
	Board_fk int NULL,
	Worker_fk int NOT NULL,
	IsSuperUser bit DEFAULT 0 NOT NULL,
	CONSTRAINT UPL_USER_PK PRIMARY KEY (pk),
	CONSTRAINT PROFILE_Worker_FK FOREIGN KEY (Worker_fk) REFERENCES uploader.WORKER(pk),
	CONSTRAINT UPL_USER_Board_FK FOREIGN KEY (Board_fk) REFERENCES uploader.BOARD(pk)
);
ALTER TABLE uploader.PROFILE WITH NOCHECK ADD CONSTRAINT PROFILE_CHECK CHECK (([IsSuperUser]=(1) AND [Board_fk] IS NULL OR [IsSuperUser]=(0) AND [Board_fk] IS NOT NULL));

-- Extended properties

EXEC sys.sp_addextendedproperty @name=N'MS_Description', @value=N'One user can have multiple profiles as longe they are in different boards', @level0type=N'Schema', @level0name=N'uploader', @level1type=N'Table', @level1name=N'PROFILE';


-- uploader.SHEET definition

-- Drop table

-- DROP TABLE uploader.SHEET;

CREATE TABLE uploader.SHEET (
	pk int IDENTITY(1,1) NOT NULL,
	Description varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	TableName varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	LastEditedBy_fk int NOT NULL,
	Active bit DEFAULT 1 NOT NULL,
	DaysToRefresh int DEFAULT 30 NOT NULL,
	Model varbinary(MAX) DEFAULT NULL NULL,
	RequestAfterUpdate varchar(MAX) COLLATE SQL_Latin1_General_CP1_CI_AI NULL,
	CONSTRAINT SHEET_PK PRIMARY KEY (pk),
	CONSTRAINT SHEET_PROFILE_FK FOREIGN KEY (LastEditedBy_fk) REFERENCES uploader.PROFILE(pk)
);


-- uploader.SHEET_META_DATA definition

-- Drop table

-- DROP TABLE uploader.SHEET_META_DATA;

CREATE TABLE uploader.SHEET_META_DATA (
	Sheet_fk int NOT NULL,
	ColumnName varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	ColumnType_fk int NOT NULL,
	Optional bit DEFAULT 0 NOT NULL,
	RegexConstraint varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI DEFAULT NULL NULL,
	LastEditedBy_fk int NOT NULL,
	pk int IDENTITY(1,1) NOT NULL,
	Description nvarchar(MAX) COLLATE SQL_Latin1_General_CP1_CI_AS NOT NULL,
	CONSTRAINT SHEET_META_DATA_PK PRIMARY KEY (pk),
	CONSTRAINT SHEET_META_DATA_UNIQUE UNIQUE (Sheet_fk,ColumnName),
	CONSTRAINT SHEET_META_DATA_COLUMN_TYPE_FK FOREIGN KEY (ColumnType_fk) REFERENCES uploader.COLUMN_TYPE(pk),
	CONSTRAINT SHEET_META_DATA_PROFILE_FK FOREIGN KEY (LastEditedBy_fk) REFERENCES uploader.PROFILE(pk),
	CONSTRAINT SHEET_META_DATA_SHEET_FK FOREIGN KEY (Sheet_fk) REFERENCES uploader.SHEET(pk)
);


-- uploader.SHEET_USED_BY_BOARD definition

-- Drop table

-- DROP TABLE uploader.SHEET_USED_BY_BOARD;

CREATE TABLE uploader.SHEET_USED_BY_BOARD (
	Sheet_fk int NOT NULL,
	Board_fk int NOT NULL,
	CONSTRAINT BOARD_SHEET_PK PRIMARY KEY (Sheet_fk,Board_fk),
	CONSTRAINT BOARD_SHEET_BOARD_FK FOREIGN KEY (Board_fk) REFERENCES uploader.BOARD(pk),
	CONSTRAINT BOARD_SHEET_SHEET_FK FOREIGN KEY (Sheet_fk) REFERENCES uploader.SHEET(pk)
);


-- uploader.UPLOAD definition

-- Drop table

-- DROP TABLE uploader.UPLOAD;

CREATE TABLE uploader.UPLOAD (
	Sheet_fk int NOT NULL,
	FileUploaded varbinary(MAX) NOT NULL,
	UploadedAt datetime DEFAULT getdate() NOT NULL,
	UploadedBy_fk int NOT NULL,
	SheetUsed varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI DEFAULT NULL NULL,
	CONSTRAINT UPLOAD_PROFILE_FK FOREIGN KEY (UploadedBy_fk) REFERENCES uploader.PROFILE(pk),
	CONSTRAINT UPLOAD_SHEET_FK FOREIGN KEY (Sheet_fk) REFERENCES uploader.SHEET(pk)
);

-- Extended properties

EXEC DIGITAL_BRA_DEV.sys.sp_addextendedproperty @name=N'MS_Description', @value=N'SheetUsed -> What excel sheet was used to load data', @level0type=N'Schema', @level0name=N'uploader', @level1type=N'Table', @level1name=N'UPLOAD';

-- uploader.CUSTOM_SQL_SCRIPT definition

-- Drop table

-- DROP TABLE uploader.CUSTOM_SQL_SCRIPT;

CREATE TABLE uploader.CUSTOM_SQL_SCRIPT (
	Sheet_fk int NOT NULL,
	RunBeforeUpdate bit DEFAULT 0 NOT NULL,
	RunAfterUpdate bit DEFAULT 0 NOT NULL,
	RunAsUpdate bit DEFAULT 0 NOT NULL,
	CustomScript varchar(MAX) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	CONSTRAINT CUSTOM_SQL_SCRIPT_SHEET_FK FOREIGN KEY (Sheet_fk) REFERENCES uploader.SHEET(pk)
);


-- uploader.[GROUP] definition

-- Drop table

-- DROP TABLE uploader.[GROUP];

CREATE TABLE uploader.[GROUP] (
	pk int IDENTITY(1,1) NOT NULL,
	Name varchar(100) COLLATE SQL_Latin1_General_CP1_CI_AI NOT NULL,
	Active bit DEFAULT 1 NOT NULL,
	BoardId int NOT NULL,
	LastEditedBy_fk int NOT NULL,
	CONSTRAINT USER_GROUPS_PK PRIMARY KEY (pk),
	CONSTRAINT GROUPS_BOARD_FK FOREIGN KEY (BoardId) REFERENCES uploader.BOARD(pk),
	CONSTRAINT GROUP_PROFILE_FK FOREIGN KEY (LastEditedBy_fk) REFERENCES uploader.PROFILE(pk)
);


-- uploader.MANAGER_PERMISSION definition

-- Drop table

-- DROP TABLE uploader.MANAGER_PERMISSION;

CREATE TABLE uploader.MANAGER_PERMISSION (
	Group_fk int NOT NULL,
	AddWorker bit DEFAULT 0 NOT NULL,
	EditWorker bit DEFAULT 0 NOT NULL,
	AddProfile bit DEFAULT 0 NOT NULL,
	RemoveProfile bit DEFAULT 0 NOT NULL,
	AddGroup bit DEFAULT 0 NOT NULL,
	RemoveGroup bit DEFAULT 0 NOT NULL,
	EditGroup bit DEFAULT 0 NOT NULL,
	EditProfileGroups bit DEFAULT 0 NOT NULL,
	ImpersonateUsers bit DEFAULT 0 NOT NULL,
	CONSTRAINT GROUP_PERMISSION_UNIQUE UNIQUE (Group_fk),
	CONSTRAINT MANAGEMENT_PERMISSION_GROUPS_FK FOREIGN KEY (Group_fk) REFERENCES uploader.[GROUP](pk)
);

-- Extended properties

EXEC sys.sp_addextendedproperty @name=N'MS_Description', @value=N'Can only be edited by super users', @level0type=N'Schema', @level0name=N'uploader', @level1type=N'Table', @level1name=N'MANAGER_PERMISSION';


-- uploader.PROFILE_GROUPS definition

-- Drop table

-- DROP TABLE uploader.PROFILE_GROUPS;

CREATE TABLE uploader.PROFILE_GROUPS (
	Profile_fk int NOT NULL,
	Group_fk int NOT NULL,
	CONSTRAINT PROFILE_GROUPS_PK PRIMARY KEY (Profile_fk,Group_fk),
	CONSTRAINT USER_GROUPS_GROUPS_FK FOREIGN KEY (Group_fk) REFERENCES uploader.[GROUP](pk),
	CONSTRAINT USER_GROUPS_UPL_USER_FK FOREIGN KEY (Profile_fk) REFERENCES uploader.PROFILE(pk)
);


-- uploader.UPLOADER_PERMISSION definition

-- Drop table

-- DROP TABLE uploader.UPLOADER_PERMISSION;

CREATE TABLE uploader.UPLOADER_PERMISSION (
	Group_fk int NOT NULL,
	Sheet_fk int NOT NULL,
	CanViewHist bit DEFAULT 0 NOT NULL,
	CanUpload bit DEFAULT 0 NOT NULL,
	LastEditedBy_fk int NOT NULL,
	CONSTRAINT UPLOADER_PERMISSION_UNIQUE UNIQUE (Group_fk,Sheet_fk),
	CONSTRAINT UPLOADER_PERMISSION_PROFILE_FK FOREIGN KEY (LastEditedBy_fk) REFERENCES uploader.PROFILE(pk),
	CONSTRAINT UPLOADER_PERMISSION_SHEET_FK FOREIGN KEY (Sheet_fk) REFERENCES uploader.SHEET(pk),
	CONSTRAINT UPLOADER_PERMISSION_USER_GROUPS_FK FOREIGN KEY (Group_fk) REFERENCES uploader.[GROUP](pk)
);

-- Triggers

-- SHEET

CREATE OR ALTER TRIGGER uploader.[TGG_SHEET]
ON uploader.SHEET
AFTER INSERT, UPDATE, DELETE
AS
BEGIN
    SET NOCOUNT ON;
 
    -- Handle INSERT
    IF EXISTS (SELECT * FROM INSERTED) AND NOT EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_SHEET
               (Sheet_fk, Description, TableName,       EditBy_fk, Active, DaysToRefresh, Model, EditAction)
        SELECT
                      pk, Description, TableName, LastEditedBy_fk, Active, DaysToRefresh, Model, 'NEW'
        FROM INSERTED
        
        RETURN;
    END
 
    -- Handle DELETE
    ELSE IF EXISTS (SELECT * FROM DELETED) AND NOT EXISTS (SELECT * FROM INSERTED)
    BEGIN
        RAISERROR('Deleting rows from table data is not allowed.', 16, 1);
    	ROLLBACK TRANSACTION;
    
    	RETURN;
    END
    
    -- Handle UPDATE
    ELSE IF EXISTS (SELECT * FROM INSERTED) AND EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_SHEET
            (Sheet_fk, Description, TableName, EditBy_fk, Active, DaysToRefresh, Model, EditAction)
        SELECT
            i.pk,
            CASE WHEN i.Description   != d.Description   OR d.Description IS NULL            
            	THEN i.Description
            	ELSE NULL END,
            CASE WHEN i.TableName     != d.TableName     OR d.TableName IS NULL           
            	THEN i.TableName           
            	ELSE NULL END,
            i.LastEditedBy_fk,
            CASE WHEN i.Active        != d.Active        OR d.Active IS NULL     
            	THEN i.Active     
            	ELSE NULL END,
            CASE WHEN i.DaysToRefresh != d.DaysToRefresh OR d.DaysToRefresh IS NULL          
            	THEN i.DaysToRefresh          
            	ELSE NULL END,
            CASE WHEN i.Model         != d.Model         OR d.Model IS NULL         
            	THEN i.Model         
            	ELSE NULL END,
            'UPDATE'
        FROM INSERTED i
        INNER JOIN DELETED d 
        	ON i.pk = d.pk
        RETURN;
    END
END

-- SHEET_META_DATA

CREATE OR ALTER TRIGGER uploader.[TGG_SHEET_META_DATA]
ON uploader.SHEET_META_DATA
AFTER INSERT, UPDATE, DELETE
AS
BEGIN
    SET NOCOUNT ON;
 
    -- Handle INSERT
    IF EXISTS (SELECT * FROM INSERTED) AND NOT EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_SHEET_META_DATA
               (SheetMetaData_fk, Sheet_fk, ColumnName, ColumnType_fk, Optional, RegexConstraint,     EditedBy_fk, EditAction)
        SELECT
                              pk, Sheet_fk, ColumnName, ColumnType_fk, Optional, RegexConstraint, LastEditedBy_fk, 'NEW'
        FROM INSERTED
        
        RETURN;
    END
 
    -- Handle DELETE
    ELSE IF EXISTS (SELECT * FROM DELETED) AND NOT EXISTS (SELECT * FROM INSERTED)
    BEGIN
        RAISERROR('Deleting rows from table data is not allowed.', 16, 1);
    	ROLLBACK TRANSACTION;
    
    	RETURN;
    END
    
    -- Handle UPDATE
    ELSE IF EXISTS (SELECT * FROM INSERTED) AND EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_SHEET_META_DATA
            (SheetMetaData_fk, Sheet_fk, ColumnName, ColumnType_fk, Optional, RegexConstraint, EditedBy_fk, EditAction)
        SELECT
            i.pk,
            CASE WHEN i.Sheet_fk   != d.Sheet_fk   OR d.Sheet_fk IS NULL            
            	THEN i.Sheet_fk
            	ELSE NULL END,
            CASE WHEN i.ColumnName   != d.ColumnName   OR d.ColumnName IS NULL            
            	THEN i.ColumnName
            	ELSE NULL END,
            CASE WHEN i.ColumnType_fk   != d.ColumnType_fk   OR d.ColumnType_fk IS NULL            
            	THEN i.ColumnType_fk
            	ELSE NULL END,
            CASE WHEN i.Optional   != d.Optional   OR d.Optional IS NULL            
            	THEN i.Optional
            	ELSE NULL END,
            CASE WHEN i.RegexConstraint   != d.RegexConstraint   OR d.RegexConstraint IS NULL            
            	THEN i.RegexConstraint
            	ELSE NULL END,
            i.LastEditedBy_fk,
            'UPDATE'
        FROM INSERTED i
        INNER JOIN DELETED d 
        	ON i.pk = d.pk
        RETURN;
    END
END

-- UPLOADER_PERMISSION

CREATE OR ALTER TRIGGER uploader.[TGG_UPLOADER_PERMISSION]
ON uploader.UPLOADER_PERMISSION
AFTER INSERT, UPDATE, DELETE
AS
BEGIN
    SET NOCOUNT ON;
 
    -- Handle INSERT
    IF EXISTS (SELECT * FROM INSERTED) AND NOT EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_UPLOADER_PERMISSION
               (Group_fk, Sheet_fk, CanViewHist, CanUpload, EditedBy_fk, ActionHist)
        SELECT
                Group_fk, Sheet_fk, CanViewHist, CanUpload, LastEditedBy_fk, 'NEW'
        FROM INSERTED
        
        RETURN;
    END
 
    -- Handle DELETE
    ELSE IF EXISTS (SELECT * FROM DELETED) AND NOT EXISTS (SELECT * FROM INSERTED)
    BEGIN
        RAISERROR('Deleting rows from table data is not allowed.', 16, 1);
    	ROLLBACK TRANSACTION;
    
    	RETURN;
    END
    
    -- Handle UPDATE
    ELSE IF EXISTS (SELECT * FROM INSERTED) AND EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_UPLOADER_PERMISSION
            (Group_fk, Sheet_fk, CanViewHist, CanUpload, EditedBy_fk, ActionHist)
        SELECT
            CASE WHEN i.Group_fk   != d.Group_fk   OR d.Group_fk IS NULL            
            	THEN i.Group_fk
            	ELSE NULL END,
            CASE WHEN i.Sheet_fk   != d.Sheet_fk   OR d.Sheet_fk IS NULL            
            	THEN i.Sheet_fk
            	ELSE NULL END,
            CASE WHEN i.CanViewHist   != d.CanViewHist   OR d.CanViewHist IS NULL            
            	THEN i.CanViewHist
            	ELSE NULL END,
            CASE WHEN i.CanUpload   != d.CanUpload   OR d.CanUpload IS NULL            
            	THEN i.CanUpload
            	ELSE NULL END,
            i.LastEditedBy_fk,
            'UPDATE'
        FROM INSERTED i
        INNER JOIN DELETED d 
        	ON i.Group_fk = d.Group_fk
        	AND i.Sheet_fk = i.Sheet_fk
        RETURN;
    END
END

-- GROUP

CREATE OR ALTER TRIGGER uploader.[TGG_GROUP]
ON uploader.[GROUP]
AFTER INSERT, UPDATE, DELETE
AS
BEGIN
    SET NOCOUNT ON;
 
    -- Handle INSERT
    IF EXISTS (SELECT * FROM INSERTED) AND NOT EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_GROUP
               (Group_fk, Name, Active,     EditedBy_fk, EditAction)
        SELECT
				      pk, Name, Active, LastEditedBy_fk, 'NEW'
        FROM INSERTED AS i
        
        RETURN;
    END
 
    -- Handle DELETE
    ELSE IF EXISTS (SELECT * FROM DELETED) AND NOT EXISTS (SELECT * FROM INSERTED)
    BEGIN
        RAISERROR('Deleting rows from table data is not allowed.', 16, 1);
    	ROLLBACK TRANSACTION;
    
    	RETURN;
    END
    
    -- Se apenas o campo LastEditedBy_fk mudar um log nao deve ser criado
    ELSE IF NOT EXISTS (
    	SELECT * FROM INSERTED i
    	INNER JOIN DELETED d
    		ON i.pk = d.pk
    	WHERE  i.Name   != d.Name
    		OR i.Active != d.Active
    )
    BEGIN
	    RETURN;
    END
    
    -- Handle UPDATE
    ELSE IF EXISTS (SELECT * FROM INSERTED) AND EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_GROUP
		    (Group_fk, Name, Active, EditedBy_fk, EditAction)
		SELECT
			i.pk,
		    CASE WHEN i.Name   != d.Name   OR d.Name IS NULL
		         THEN i.Name
		         ELSE NULL END,
		    CASE WHEN i.Active != d.Active OR d.Active IS NULL
		         THEN i.Active
		         ELSE NULL END,
		    i.LastEditedBy_fk,
		    'UPDATE'
		FROM INSERTED i
		INNER JOIN DELETED d 
        	ON i.pk = d.pk
        	
        RETURN;
    END
END

-- MANAGER_PERMISSION

CREATE OR ALTER TRIGGER uploader.[TGG_MANAGER_PERMISSION]
ON uploader.MANAGER_PERMISSION
AFTER INSERT, UPDATE, DELETE
AS
BEGIN
    SET NOCOUNT ON;
 
    -- Handle INSERT
    IF EXISTS (SELECT * FROM INSERTED) AND NOT EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_GROUP
               (Group_fk,      AddWorker,   EditWorker,        AddProfile,         RemoveProfile,   AddGroup,
                RemoveGroup,   EditGroup,   EditProfileGroups, ImpersonateUsers,   EditedBy_fk,     EditAction)
        SELECT
                i.Group_fk,    i.AddWorker, i.EditWorker,        i.AddProfile,       i.RemoveProfile, i.AddGroup, 
                i.RemoveGroup, i.EditGroup, i.EditProfileGroups, i.ImpersonateUsers, g.LastEditedBy_fk, 'NEW'
        FROM INSERTED AS i
        INNER JOIN uploader.[GROUP] AS g
        	ON i.Group_fk = g.pk
        
        RETURN;
    END
 
    -- Handle DELETE
    ELSE IF EXISTS (SELECT * FROM DELETED) AND NOT EXISTS (SELECT * FROM INSERTED)
    BEGIN
        RAISERROR('Deleting rows from table data is not allowed.', 16, 1);
    	ROLLBACK TRANSACTION;
    
    	RETURN;
    END
    
    -- Handle UPDATE
    ELSE IF EXISTS (SELECT * FROM INSERTED) AND EXISTS (SELECT * FROM DELETED)
    BEGIN
        INSERT INTO uploader.HIST_GROUP
		    (Group_fk, AddWorker, EditWorker, AddProfile, RemoveProfile, AddGroup,
		     RemoveGroup, EditGroup, EditProfileGroups, ImpersonateUsers, EditedBy_fk, EditAction)
		SELECT
		    CASE WHEN i.Group_fk         != d.Group_fk         OR d.Group_fk IS NULL
		         THEN i.Group_fk
		         ELSE NULL END,
		    CASE WHEN i.AddWorker        != d.AddWorker        OR d.AddWorker IS NULL
		         THEN i.AddWorker
		         ELSE NULL END,
		    CASE WHEN i.EditWorker       != d.EditWorker       OR d.EditWorker IS NULL
		         THEN i.EditWorker
		         ELSE NULL END,
		    CASE WHEN i.AddProfile       != d.AddProfile       OR d.AddProfile IS NULL
		         THEN i.AddProfile
		         ELSE NULL END,
		    CASE WHEN i.RemoveProfile    != d.RemoveProfile    OR d.RemoveProfile IS NULL
		         THEN i.RemoveProfile
		         ELSE NULL END,
		    CASE WHEN i.AddGroup         != d.AddGroup         OR d.AddGroup IS NULL
		         THEN i.AddGroup
		         ELSE NULL END,
		    CASE WHEN i.RemoveGroup      != d.RemoveGroup      OR d.RemoveGroup IS NULL
		         THEN i.RemoveGroup
		         ELSE NULL END,
		    CASE WHEN i.EditGroup        != d.EditGroup        OR d.EditGroup IS NULL
		         THEN i.EditGroup
		         ELSE NULL END,
		    CASE WHEN i.EditProfileGroups != d.EditProfileGroups OR d.EditProfileGroups IS NULL
		         THEN i.EditProfileGroups
		         ELSE NULL END,
		    CASE WHEN i.ImpersonateUsers != d.ImpersonateUsers OR d.ImpersonateUsers IS NULL
		         THEN i.ImpersonateUsers
		         ELSE NULL END,
		    g.LastEditedBy_fk,
		    'UPDATE'
		FROM INSERTED i
		INNER JOIN DELETED d 
        	ON i.Group_fk = d.Group_fk
        INNER JOIN uploader.[GROUP] AS g
        	ON i.Group_fk = g.pk
        	
        RETURN;
    END
END
