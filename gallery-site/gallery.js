document.addEventListener("DOMContentLoaded", () => {
  loadGallery();

  const uploadBtn = document.getElementById("uploadButton");
  const fileInput = document.getElementById("fileInput");

  // 1) When the user clicks the button, "click" the hidden file input
  uploadBtn.addEventListener("click", () => {
    fileInput.click();
  });

  // 2) When the user actually picks a file, do the upload
  fileInput.addEventListener("change", () => {
    if (fileInput.files[0]) {
      uploadImage(fileInput.files[0]);
      // Clear fileInput so subsequent clicks re-trigger:
    }
  });
});

async function uploadImage(file) {
  try {
    const key = `uploads/${file.name}`;

    // Get a pre-signed URL
    const response = await fetch(
      `http://localhost:3000/get-upload-url?key=${encodeURIComponent(key)}`
    );
    if (!response.ok) {
      alert("Failed to get pre-signed URL");
      return;
    }
    const presignedUrl = await response.text();

    // PUT the file directly to S3
    const putResp = await fetch(presignedUrl, {
      method: "PUT",
      body: file,
      // headers: { "Content-Type": file.type }
    });
    if (!putResp.ok) {
      alert("Upload failed!");
      return;
    }

    alert("Upload succeeded!");
    loadGallery();
  } catch (error) {
    console.error("Error uploading image:", error);
    alert("Error uploading image.");
  }
}

// Call /list-images
async function loadGallery() {
  try {
    const response = await fetch("http://localhost:3000/list-images");
    if (!response.ok) throw new Error("Failed to fetch image list");

    const data = await response.json();
    const imageUrls = data.images || [];

    const gallery = document.getElementById("gallery");
    gallery.innerHTML = "";

    imageUrls.forEach(url => {
      const a = document.createElement("a");
      a.href = url;
      a.download = "";

      const img = document.createElement("img");
      img.src = url;
      img.alt = "Processed image";

      a.appendChild(img);
      gallery.appendChild(a);
    });
  } catch (err) {
    console.error("Error loading gallery:", err);
  }
}