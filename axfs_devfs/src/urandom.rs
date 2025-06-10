use axfs_vfs::{VfsNodeAttr, VfsNodeOps, VfsNodePerm, VfsNodeType, VfsResult};
use core::sync::atomic::{AtomicU64, Ordering};

/// A urandom device behaves like `/dev/urandom`.
///
/// It produces random bytes when read.
pub struct UrandomDev {
    seed: AtomicU64,
}

impl UrandomDev {
    pub fn new() -> Self {
        // 在实际环境中应该使用更好的初始种子，例如从系统时钟等来源获取
        Self {
            seed: AtomicU64::new(0x1234_5678_9abc_def0),
        }
    }
    
    // 简单的伪随机数生成算法
    fn next_u64(&self) -> u64 {
        let mut x = self.seed.load(Ordering::Relaxed);
        // 使用简单的XorShift算法
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed.store(x, Ordering::Relaxed);
        x
    }
}

impl Default for UrandomDev {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsNodeOps for UrandomDev {
    fn get_attr(&self) -> VfsResult<VfsNodeAttr> {
        Ok(VfsNodeAttr::new(
            VfsNodePerm::default_file(),
            VfsNodeType::CharDevice,
            0,
            0,
        ))
    }

    fn read_at(&self, _offset: u64, buf: &mut [u8]) -> VfsResult<usize> {
        // 以8字节(u64)为单位填充随机数据
        for chunk in buf.chunks_mut(8) {
            let random_value = self.next_u64();
            let bytes = random_value.to_ne_bytes();
            for (i, byte) in chunk.iter_mut().enumerate() {
                if i < bytes.len() {
                    *byte = bytes[i];
                }
            }
        }
        Ok(buf.len())
    }

    fn write_at(&self, _offset: u64, buf: &[u8]) -> VfsResult<usize> {
        // 写入被忽略，但返回成功写入的字节数
        Ok(buf.len())
    }

    fn truncate(&self, _size: u64) -> VfsResult {
        Ok(())
    }

    axfs_vfs::impl_vfs_non_dir_default! {}
}