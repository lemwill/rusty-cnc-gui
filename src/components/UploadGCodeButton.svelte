<script>
  let selectedFile;
  const uploadFile = () => {
    const formData = new FormData();
    formData.append("file", selectedFile);

    // Make a POST request to your backend
    fetch("http://localhost:8000/upload", {
      method: "POST",
      body: formData,
    });
  };
</script>

<input class="input" type="file" bind:files={selectedFile} />
<button class="btn variant-filled" on:click={uploadFile}>Upload</button>
