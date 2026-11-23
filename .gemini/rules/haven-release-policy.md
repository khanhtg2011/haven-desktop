# Haven Release Policy: Pre-Release Only

## ⚠️ Quy tắc bắt buộc về phát hành (Release Policy)
1. **Toàn bộ ứng dụng viết bằng Rust (`haven-native`) PHẢI luôn được đánh dấu và phát hành dưới dạng `Pre-Release` (Alpha / Beta / Pre-release).**
2. **Tuyệt đối không chuyển sang bản phát hành chính thức (Official/Stable Release) khi chưa có sự kiểm thử và duyệt đồng ý trực tiếp từ người dùng (User Approval).**
3. Trong các workflow GitHub Actions:
   - Thuộc tính `prerelease` luôn luôn phải đặt là `true` (`prerelease: true`).
   - Định dạng tag / version luôn mang hậu tố `-pre`, `-alpha`, hoặc `-beta` (ví dụ: `v0.2.0-pre.1`).
   - Ghi chú phát hành phải có thông báo rõ ràng: *"Bản thử nghiệm (Pre-Release). Bản chính thức cần người dùng duyệt hoàn thành."*
