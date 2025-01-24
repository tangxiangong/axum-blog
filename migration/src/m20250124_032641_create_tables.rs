use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

/// 网站基本信息
#[derive(DeriveIden)]
enum Website {
    Table,
    Title,
    Subtitle,
    Description,
    Logo,
    Favicon,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Admin {
    Table,
    Id,
    Name,
    Nickname,
    Password,
    Email,
    Avatar,
    Github,
    Wechat,
    QQ,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Jwt {
    Table,
    Id,
    #[sea_orm(iden = "user_uuid")]
    UserUuid,
    Token,
    #[sea_orm(iden = "expire_duration")]
    ExpireDuration,
    #[sea_orm(iden = "expire_at")]
    ExpireAt,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Category {
    Table,
    Id,
    Name,
    #[sea_orm(iden = "parent_id")]
    ParentId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tag {
    Table,
    Id,
    Name,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Article {
    Table,
    Id,
    Title,
    Summary,
    Content,
    Views,
    Publish,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Comment {
    Table,
    Id,
    Article,
    Name,
    Email,
    #[sea_orm(iden = "parent_id")]
    ParentId,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "article_category")]
enum ArticleCategory {
    Table,
    #[sea_orm(iden = "article_id")]
    ArticleId,
    #[sea_orm(iden = "category_id")]
    CategoryId,
}

#[derive(DeriveIden)]
#[sea_orm(table_name = "article_tag")]
enum ArticleTag {
    Table,
    #[sea_orm(iden = "article_id")]
    ArticleId,
    #[sea_orm(iden = "tag_id")]
    TagId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Website::Table)
                    .comment("网站基本信息")
                    .if_not_exists()
                    .col(
                        string(Website::Title)
                            .string_len(32)
                            .comment("网站标题")
                            .primary_key(),
                    )
                    .col(
                        string_null(Website::Subtitle)
                            .string_len(64)
                            .comment("网站副标题"),
                    )
                    .col(
                        string_null(Website::Description)
                            .string_len(64)
                            .comment("网站描述"),
                    )
                    .col(string_null(Website::Logo).comment("网站 Logo"))
                    .col(string_null(Website::Favicon).comment("网站 Favicon"))
                    .col(
                        timestamp(Website::CreatedAt)
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Website::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Admin::Table)
                    .comment("管理员表")
                    .if_not_exists()
                    .col(string(Admin::Id).string_len(128).primary_key())
                    .col(
                        string(Admin::Name)
                            .string_len(16)
                            .comment("管理员名称")
                            .unique_key(),
                    )
                    .col(blob(Admin::Password).comment("密码"))
                    .col(string_null(Admin::Nickname).string_len(16).comment("昵称"))
                    .col(string_null(Admin::Email).string_len(128).comment("邮箱"))
                    .col(string_null(Admin::Github).comment("Github"))
                    .col(string_null(Admin::Wechat).comment("微信"))
                    .col(string_null(Admin::QQ).comment("QQ"))
                    .col(string_null(Admin::Avatar).comment("头像"))
                    .col(
                        timestamp(Admin::CreatedAt)
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Admin::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Jwt::Table)
                    .comment("存放JWT, 需要定时清理")
                    .if_not_exists()
                    .col(pk_auto(Jwt::Id).unsigned().comment("ID"))
                    .col(string(Jwt::UserUuid).string_len(128).comment("用户UUID"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-jwt-user")
                            .from(Jwt::Table, Jwt::UserUuid)
                            .to(Admin::Table, Admin::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(string(Jwt::Token).string_len(512).comment("token"))
                    .col(
                        timestamp(Jwt::ExpireAt)
                            .default(Expr::current_timestamp())
                            .comment("过期时间"),
                    )
                    .col(big_integer(Jwt::ExpireDuration).comment("过期时长, 单位秒"))
                    .col(
                        timestamp(Jwt::CreatedAt)
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Jwt::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .comment("分类表")
                    .if_not_exists()
                    .col(pk_auto(Category::Id).unsigned().comment("分类ID"))
                    .col(
                        string(Category::Name)
                            .string_len(32)
                            .unique_key()
                            .not_null()
                            .comment("分类名称"),
                    )
                    .col(
                        integer(Category::ParentId)
                            .unsigned()
                            .null()
                            .comment("父分类"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-category-parent")
                            .from(Category::Table, Category::ParentId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        timestamp(Category::CreatedAt)
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Category::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tag::Table)
                    .comment("标签表")
                    .if_not_exists()
                    .col(pk_auto(Tag::Id).unsigned().comment("标签ID"))
                    .col(
                        string(Tag::Name)
                            .string_len(32)
                            .not_null()
                            .unique_key()
                            .comment("标签名称"),
                    )
                    .col(
                        timestamp(Tag::CreatedAt)
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Tag::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .comment("文章表")
                    .if_not_exists()
                    .col(pk_auto(Article::Id).unsigned().comment("文章ID"))
                    .col(
                        string(Article::Title)
                            .string_len(128)
                            .not_null()
                            .comment("文章标题"),
                    )
                    .col(string_null(Article::Summary).comment("文章摘要"))
                    .col(text(Article::Content).not_null().comment("文章内容"))
                    .col(
                        integer(Article::Views)
                            .unsigned()
                            .default(0)
                            .comment("浏览量"),
                    )
                    .col(boolean(Article::Publish).default(true).comment("是否发布"))
                    .col(
                        timestamp(Article::CreatedAt)
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        timestamp(Article::UpdatedAt)
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .comment("评论表")
                    .if_not_exists()
                    .col(pk_auto(Comment::Id).unsigned().comment("评论ID"))
                    .col(
                        integer(Comment::Article)
                            .unsigned()
                            .not_null()
                            .comment("评论所在文章ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-article")
                            .from(Comment::Table, Comment::Article)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        string(Comment::Name)
                            .string_len(32)
                            .not_null()
                            .comment("评论者名称"),
                    )
                    .col(string_null(Comment::Email))
                    .col(integer(Comment::ParentId).unsigned().comment("父评论ID"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comment-parent")
                            .from(Comment::Table, Comment::ParentId)
                            .to(Comment::Table, Comment::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(timestamp(Comment::CreatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ArticleCategory::Table)
                    .comment("文章分类关联表")
                    .if_not_exists()
                    .col(
                        integer(ArticleCategory::ArticleId)
                            .not_null()
                            .unsigned()
                            .comment("对应文章ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-article_category-article")
                            .from(ArticleCategory::Table, ArticleCategory::ArticleId)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        integer(ArticleCategory::CategoryId)
                            .unsigned()
                            .not_null()
                            .comment("对应分类ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-article_category-category")
                            .from(ArticleCategory::Table, ArticleCategory::CategoryId)
                            .to(Category::Table, Category::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(ArticleCategory::ArticleId)
                            .col(ArticleCategory::CategoryId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ArticleTag::Table)
                    .comment("文章标签关联表")
                    .if_not_exists()
                    .col(
                        integer(ArticleTag::ArticleId)
                            .unsigned()
                            .not_null()
                            .comment("对应文章ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-article_tag-article")
                            .from(ArticleTag::Table, ArticleTag::ArticleId)
                            .to(Article::Table, Article::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        integer(ArticleTag::TagId)
                            .unsigned()
                            .not_null()
                            .comment("对应标签ID"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-article_tag-tag")
                            .from(ArticleTag::Table, ArticleTag::TagId)
                            .to(Tag::Table, Tag::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(ArticleTag::ArticleId)
                            .col(ArticleTag::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ArticleCategory::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ArticleTag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Tag::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Jwt::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Admin::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Website::Table).to_owned())
            .await?;

        Ok(())
    }
}
