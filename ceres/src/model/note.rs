use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// /// Note information
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct NoteInfo {
//     pub id: String,
//     pub title: String,
//     pub created_at: String,
//     pub last_activity_at: String,
//     pub content_updated_at: String,
//     pub comments_count: i64,
//     pub resolved_comments_count: i64,
//     pub channel_name: String,
//     pub presence_channel_name: String,
//     pub description_thumbnail_base_url: Option<String>,
//     pub public_visibility: bool,
//     pub non_member_views_count: i64,
//     pub description_html: String,
//     pub description_state: Option<String>,
//     pub project: Option<Project>,
//     pub follow_ups: Vec<FollowUp>,
//     pub type_name: String,
//     pub url: String,
//     pub public_share_url: String,
//     pub project_permission: ProjectPermission,
//     pub member: OrganizationMember,
//     pub viewer_is_author: bool,
//     pub viewer_can_comment: bool,
//     pub viewer_can_edit: bool,
//     pub viewer_can_delete: bool,
//     pub viewer_has_favorited: bool,
//     pub latest_commenters: Vec<OrganizationMember>,
//     pub permitted_users: Vec<Permission>,
//     pub project_pin_id: Option<String>,
//     pub resource_mentions: Vec<ResourceMention>,
// }

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateRequest {
    pub description_html: String,
    pub description_state: String,
    pub description_schema_version: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ShowResponse {
    #[serde(rename = "id")]
    pub public_id: String,

    pub description_schema_version: i32,

    #[serde(rename = "description_state", skip_serializing_if = "Option::is_none")]
    pub description_state: Option<String>,

    pub description_html: String,
}

// /// Query parameters for getting notes
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct GetNotesParams {
//     /// Cursor for pagination
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub after: Option<String>,
//     /// Max number of items to return
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub limit: Option<u64>,
//     /// Search query term
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub q: Option<String>,
//     /// Ordering information
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub order: Option<Order>,
//     /// Organization slug
//     pub org_slug: String,
// }

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct GetNotesRes {
//     pub next_cursor: Option<String>,
//     pub prev_cursor: Option<String>,
//     pub data: Vec<NoteInfo>,
// }

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct PutNote {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub title: Option<String>
// }

// /// ---------- Notes related models ----------
// /// ---------- Ordering options ----------
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct Order {
//     /// Field to order by: created_at | last_activity_at
//     pub by: OrderBy,
//     /// Direction: asc | desc
//     pub direction: OrderDirection,
// }

// /// Allowed fields for ordering
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum OrderBy {
//     CreatedAt,
//     LastActivityAt,
// }

// /// Order direction
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// #[serde(rename_all = "lowercase")]
// pub enum OrderDirection {
//     Asc,
//     Desc,
// }

// /// ---------- project permissions ----------
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// #[serde(rename_all = "lowercase")]
// pub enum ProjectPermission {
//     None,
//     View,
//     Edit,
// }

// /// ---------- Organization member information ----------
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct OrganizationMember {
//     /// Member ID
//     pub id: String,
//     /// Member role
//     pub role: OrganizationMemberRole,
//     /// Creation time
//     pub created_at: String,
//     /// Whether member is deactivated
//     pub deactivated: bool,
//     /// Whether is part of the organization
//     pub is_organization_member: bool,
//     /// Associated user information
//     pub user: User,
//     /// Membership status
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub status: Option<OrganizationMembershipStatus>,
// }

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// #[serde(rename_all = "lowercase")]
// pub enum OrganizationMemberRole {
//     Admin,
//     Member,
//     Viewer,
//     Guest,
// }

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// #[serde(rename_all = "snake_case")]
// pub enum OrganizationMembershipStatus {
//     Pending,
//     Active,
//     Suspended,
// }

// /// ---------- Permission information ----------
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct Permission {
//     /// Permission ID
//     pub id: String,
//     /// Associated user
//     pub user: User,
//     /// Action allowed
//     pub action: PermissionAction,
// }

// /// Enum for permission action
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// #[serde(rename_all = "lowercase")]
// pub enum PermissionAction {
//     View,
//     Edit,
// }

// /// ---------- Resource mention information ----------
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct ResourceMention {
//     /// Resource mention ID
//     pub id: String,
//     /// Associated post
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub post: Option<ResourceMentionPost>,
//     /// Associated call
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub call: Option<ResourceMentionCall>,
//     /// Associated note
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub note: Option<ResourceMentionNote>,
//     /// Type name
//     pub type_name: String,
// }

// /// Post information for resource mention
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct ResourceMentionPost {
//     /// Post ID
//     pub id: String,
//     /// Post title
//     pub title: String,
//     /// Creation time
//     pub created_at: String,
//     /// Published time (optional)
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub published_at: Option<String>,
//     /// Post URL
//     pub url: String,
// }

// /// Call information for resource mention
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct ResourceMentionCall {
//     /// Call ID
//     pub id: String,
//     /// Call title
//     pub title: String,
//     /// Creation time
//     pub created_at: String,
//     /// Call URL
//     pub url: String,
// }

// /// Note information for resource mention
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct ResourceMentionNote {
//     /// Note ID
//     pub id: String,
//     /// Note title
//     pub title: String,
//     /// Creation time
//     pub created_at: String,
//     /// Note URL
//     pub url: String,
// }

// /// ---------- FollowUp ----------
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct FollowUp {
//     pub id: String,
//     pub show_at: String,
//     pub inbox_key: String,
//     pub organization_slug: String,
//     pub member: OrganizationMember,
//     pub subject: FollowUpSubject,
//     pub target: NotificationTarget,
//     pub summary_blocks: Vec<SummaryBlock>,
//     pub belongs_to_viewer: bool,
//     pub type_name: String,
// }

// /// Summary Block
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct SummaryBlock {
//     pub text: Option<SummaryBlockText>,
//     pub img: Option<SummaryBlockImg>,
// }

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct SummaryBlockText {
//     pub content: String,
//     #[serde(default)]
//     pub bold: Option<bool>,
//     #[serde(default)]
//     pub nowrap: Option<bool>,
// }

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct SummaryBlockImg {
//     pub src: String,
//     pub alt: String,
// }

// /// FollowUp Subject
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct FollowUpSubject {
//     pub id: String,
//     #[serde(rename = "type")]
//     pub type_name: String,
//     pub body_preview: String,
//     pub member: Option<OrganizationMember>,
//     pub title: Option<String>,
// }

// /// Notification Target
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct NotificationTarget {
//     pub id: String,
//     #[serde(rename = "type")]
//     pub type_name: String,
//     pub title: String,
//     pub project: Option<MiniProject>,
//     pub resolved: bool,
// }

// /// ---------- User Information ----------
// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct User {
//     pub id: String,
//     pub avatar_url: String,
//     pub avatar_urls: AvatarUrls,
//     pub cover_photo_url: Option<String>,
//     pub email: String,
//     pub username: String,
//     pub display_name: String,
//     pub system: bool,
//     pub integration: bool,
//     pub notifications_paused: bool,
//     pub notification_pause_expires_at: Option<String>,
//     pub timezone: Option<String>,
//     pub logged_in: bool,
//     pub type_name: String,
// }

// #[derive(Debug, Serialize, Deserialize, ToSchema)]
// pub struct AvatarUrls {
//     pub xs: String,
//     pub sm: String,
//     pub base: String,
//     pub lg: String,
//     pub xl: String,
//     pub xxl: String,
// }
