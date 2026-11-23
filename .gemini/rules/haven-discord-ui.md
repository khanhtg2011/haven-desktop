# Haven Discord UI & Layout Rules

Quy tắc bắt buộc khi bảo trì và phát triển giao diện hệ sinh thái Haven (Web, Desktop Tauri, Android APK, Docker Container):

---

## 1. Tính toàn vẹn cấu trúc DOM của Sidebar (`app.html`)
- **Phạm vi thẻ `<aside class="sidebar">`:** Phải bao trọn toàn bộ 3 phân vùng:
  1. `.sidebar-header`: Chỉ chứa logo thương hiệu (`.brand`) và nút đóng mobile drawer (`#mobile-sidebar-close`). Tuyệt đối không chứa `.user-bar` hay các nút trùng lặp.
  2. `.sidebar-mod-container`: Chứa danh sách kênh (`#channel-list`), server bubbles và direct messages (`#dm-pane`).
  3. `.sidebar-bottom`: Chứa `#voice-bar`, `#theme-popup` và DUY NHẤT một khối `.user-bar`.
- **Cân bằng thẻ đóng `</div>`:** Thẻ `aside.sidebar` phải có số lượng thẻ mở `<div` bằng chính xác số lượng thẻ đóng `</div` khi đến `</aside>` (Div Depth = 0). Tuyệt đối không đóng thừa `</div>` ở header gây vỡ drawer trên điện thoại di động.
- **Không duplicate ID:** Các ID `#user-bar-avatar`, `#current-user`, `#login-name`, `#mentions-btn`, `#rename-btn`, `#logout-btn` chỉ được xuất hiện đúng 1 lần trong toàn bộ tài liệu HTML.

---

## 2. Chuẩn hóa Khung soạn thảo (Composer Parity)
- Khung `#message-input-area` chỉ hiển thị chuẩn 4 thành phần theo thứ tự flex:
  1. `order: 1` — `#upload-btn`: Nút tải tệp tròn, biểu tượng `+` đơn nhất.
  2. `order: 2` — `#message-input`: Khung nhập văn bản mở rộng (`flex: 1 1 auto`), placeholder chuẩn tiếng Việt: `"Nhập tin nhắn... (/ cho lệnh)"`.
  3. `order: 3` — `#emoji-btn`: Nút mở khay emoji.
  4. `order: 4` — `#send-btn`: Nút gửi tin nhắn tròn màu tím Discord (`#5865F2`).
- **Triệt tiêu nút thừa (Zero Clutter):**
  - Không bao giờ hiển thị đồng thời `#composer-plus-btn` và `#upload-btn` (nguyên nhân gây lỗi 2 nút `+` cạnh nhau).
  - Toàn bộ các nút phụ trợ (`#gif-btn`, `#poll-btn`, `#time-btn`, `#format-btn`, `#voice-btn`, `#self-destruct-btn`, `#silent-btn`, `#highlight-btn`, `#burn-btn`) phải được gom vào container ẩn `#composer-stashed-buttons` (`display: none !important; width: 0; height: 0;`) để giữ nguyên liên kết sự kiện JavaScript mà không chiếm diện tích hiển thị.

---

## 3. Thao tác Vào Voice Tức thì (Voice Quick Actions)
- Trên máy tính (Desktop/Web PC): Nhấn đúp chuột (`dblclick`) vào tên kênh để tự động chuyển kênh và kết nối phòng thoại (`_joinVoice()`).
- Trên thiết bị di động (Mobile Web/APK): Chạm đúp (`touchend` cách `touchstart` từ 80ms đến 450ms, độ lệch ngón tay `dx, dy <= 12px`, cờ `passive: true`) để tự động chuyển kênh và kết nối phòng thoại.

---

## 4. Tự động Chọn Kênh & Quyền Thành viên Kênh (Channel Auto-Join)
- **Tự động chuyển kênh khi mở app:** Trong `app-socket-channels.js`, nếu `!this.currentChannel` và có danh sách kênh công khai, hệ thống phải tự động gọi `switchChannel(publicChannel.code)` để người dùng không bao giờ bị rơi vào màn hình trống `"Chào mừng bạn đến với Haven"`.
- **Cơ sở dữ liệu:** Cấu hình `server_settings.default_join_channels` phải luôn chứa ID các kênh công khai (`1,2,4`). Mọi người dùng mới hoặc hiện có phải được gán quyền trong bảng `channel_members` cho các kênh này.

---

## 5. Kiểm soát Cache & Triển khai Stylesheet
- Bất cứ khi nào cập nhật CSS, phải tăng version query string `?v=X.X.X` đồng bộ tại `app.html` và toàn bộ các dòng `@import` trong `style.css` (ví dụ `?v=5.8.0`).
- Tệp `discord-theme.css` phải luôn được import ở dòng cuối cùng của `style.css` để đảm bảo quyền ưu tiên ghi đè màu sắc và layout trên mọi thiết bị.

---

## 6. Vô hiệu hóa Thành phần Rác Braid Layout
- Plugin `BraidLayout.plugin.js` không được phép khởi tạo `.sidebar-bottom-bar`, `#braid-more-btn`, `.braid-more-wrap` hay các nút nổi `#braid-return-pill`. Tất cả tính năng người dùng phải nằm cố định tại `.user-bar` ở góc dưới bên trái sidebar.

---

## 7. Quy trình Kiểm thử Bắt buộc Trước khi Đóng Gói / Triển khai
- Chạy bộ kiểm thử tự động:
  ```bash
  npm test
  ```
- Toàn bộ các test case trong `test/discord-ui.test.js` phải vượt qua 100% trước khi commit hoặc deploy lên VPS.
