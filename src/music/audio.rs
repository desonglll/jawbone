use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Serialize;
use std::fs::DirEntry;
use std::net::Ipv4Addr;
use std::path::Path;
use std::{fs, io};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    pub uuid: Uuid,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub duration: u64,
    pub stream: Option<String>,
}

impl Audio {
    pub fn from_dir_entry(dir_entry: DirEntry) -> Self {
        let uuid = Uuid::new_v4();
        let path_buf = dir_entry.path();
        let path = path_buf.to_str().unwrap().to_string();
        let name = path_buf.file_name().unwrap().to_str().unwrap().to_string();
        let size = dir_entry.metadata().unwrap().len();
        let duration = dir_entry.metadata().unwrap().len();

        Audio {
            uuid,
            path,
            name,
            size,
            duration,
            stream: None,
        }
    }

    pub fn set_stream(&mut self, host: Ipv4Addr, port: u16, stream_entry: &str) {
        self.stream = Some(format!("/{stream_entry}/{}", self.path));
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioList(Vec<Audio>);
#[derive(Debug, Clone, Serialize)]
pub struct EncodedAudioList(Vec<Audio>);
impl AsMut<EncodedAudioList> for EncodedAudioList {
    fn as_mut(&mut self) -> &mut EncodedAudioList {
        self
    }
}
impl AudioList {
    pub fn encode(&mut self) -> EncodedAudioList {
        let encoded_list: Vec<Audio> = self
            .0
            .iter_mut()
            .map(|a| {
                let encoded_path =
                    utf8_percent_encode(a.path.as_str(), NON_ALPHANUMERIC).to_string();
                Audio {
                    path: encoded_path,
                    ..a.clone()
                }
            })
            .collect();
        EncodedAudioList(encoded_list)
    }
}
impl EncodedAudioList {
    pub fn decode(&self) -> AudioList {
        let decoded_list: Vec<Audio> = self
            .0
            .iter()
            .map(|a| {
                let decoded_path = percent_decode(a.path.as_bytes())
                    .decode_utf8()
                    .unwrap()
                    .to_string();
                Audio {
                    path: decoded_path,
                    ..a.clone()
                }
            })
            .collect();
        AudioList(decoded_list)
    }
    pub fn set_stream(&mut self, host: Ipv4Addr, port: u16, stream_entry: &str) {
        self.0.iter_mut().for_each(|audio| {
            audio.set_stream(host, port, stream_entry);
        })
    }
}
pub fn list(dir: &str) -> AudioList {
    let mut audio_files = Vec::new();
    // 调用修改后的递归方法
    visit_dirs(Path::new(dir), &mut audio_files).expect("Failed to read directory");

    AudioList(audio_files)
}

fn visit_dirs(dir: &Path, audio_files: &mut Vec<Audio>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // 如果是目录，递归调用
                visit_dirs(&path, audio_files)?;
            } else {
                // 检查文件扩展名是否为音频文件
                if let Some(extension) = path.extension() {
                    if let Some(ext) = extension.to_str() {
                        if matches!(ext, "mp3" | "wav" | "ogg") {
                            audio_files.push(Audio::from_dir_entry(entry));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

impl AsMut<AudioList> for AudioList {
    fn as_mut(&mut self) -> &mut AudioList {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::music::audio::list;

    #[test]
    fn test_audio_files() {
        let audio_list = list("/Users/mikeshinoda/Music/网易云音乐").as_mut();
        let encoded_audio_list = audio_list.encode();
        println!("{:?}", encoded_audio_list);
    }
}
