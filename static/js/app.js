document.addEventListener('DOMContentLoaded', () => {
    // DOM Elements
    const dropArea = document.getElementById('dropArea');
    const fileInput = document.getElementById('fileInput');
    const fileSelectBtn = document.getElementById('fileSelectBtn');
    const fileInfo = document.getElementById('fileInfo');
    const fileName = document.getElementById('fileName');
    const fileSize = document.getElementById('fileSize');
    const convertBtn = document.getElementById('convertBtn');
    const cancelBtn = document.getElementById('cancelBtn');
    const progressContainer = document.getElementById('progressContainer');
    const progress = document.getElementById('progress');
    const progressText = document.getElementById('progressText');
    const resultContainer = document.getElementById('resultContainer');
    const resultSuccess = document.getElementById('resultSuccess');
    const resultError = document.getElementById('resultError');
    const errorMessage = document.getElementById('errorMessage');
    const downloadBtn = document.getElementById('downloadBtn');
    const tryAgainBtn = document.getElementById('tryAgainBtn');
    
    let selectedFile = null;
    
    // Add event listeners for drag and drop
    ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
        dropArea.addEventListener(eventName, preventDefaults, false);
    });
    
    function preventDefaults(e) {
        e.preventDefault();
        e.stopPropagation();
    }
    
    ['dragenter', 'dragover'].forEach(eventName => {
        dropArea.addEventListener(eventName, highlight, false);
    });
    
    ['dragleave', 'drop'].forEach(eventName => {
        dropArea.addEventListener(eventName, unhighlight, false);
    });
    
    function highlight() {
        dropArea.classList.add('highlight');
    }
    
    function unhighlight() {
        dropArea.classList.remove('highlight');
    }
    
    // Handle dropped files
    dropArea.addEventListener('drop', handleDrop, false);
    
    function handleDrop(e) {
        const dt = e.dataTransfer;
        const files = dt.files;
        
        if (files.length > 0) {
            handleFiles(files[0]);
        }
    }
    
    // Handle file selection via button
    fileSelectBtn.addEventListener('click', () => {
        fileInput.click();
    });
    
    fileInput.addEventListener('change', () => {
        if (fileInput.files.length > 0) {
            handleFiles(fileInput.files[0]);
        }
    });
    
    // Handle selected file
    function handleFiles(file) {
        // Check if the file is a .mov file
        if (!file.name.toLowerCase().endsWith('.mov')) {
            alert('Please select a .MOV file');
            return;
        }
        
        selectedFile = file;
        
        // Display file info
        fileName.textContent = file.name;
        fileSize.textContent = formatFileSize(file.size);
        
        // Show file info and hide drop area
        dropArea.style.display = 'none';
        fileInfo.style.display = 'block';
    }
    
    // Format file size
    function formatFileSize(bytes) {
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
        if (bytes === 0) return '0 Byte';
        const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
        return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
    }
    
    // Cancel button event
    cancelBtn.addEventListener('click', resetUI);
    
    function resetUI() {
        selectedFile = null;
        fileInput.value = '';
        dropArea.style.display = 'block';
        fileInfo.style.display = 'none';
        progressContainer.style.display = 'none';
        resultContainer.style.display = 'none';
        resultSuccess.style.display = 'none';
        resultError.style.display = 'none';
    }
    
    // Convert button event
    convertBtn.addEventListener('click', convertFile);
    
    function convertFile() {
        if (!selectedFile) return;
        
        // Show progress
        fileInfo.style.display = 'none';
        progressContainer.style.display = 'block';
        
        // Simulate progress (actual progress isn't available from the server)
        let progressValue = 0;
        const progressInterval = setInterval(() => {
            if (progressValue < 90) {
                progressValue += Math.random() * 10;
                updateProgress(progressValue);
            }
        }, 300);
        
        // Create form data
        const formData = new FormData();
        formData.append('file', selectedFile);
        
        // Send to server
        fetch('/convert', {
            method: 'POST',
            body: formData
        })
        .then(response => response.json())
        .then(data => {
            clearInterval(progressInterval);
            updateProgress(100);
            
            // Wait a bit to show 100% progress
            setTimeout(() => {
                progressContainer.style.display = 'none';
                resultContainer.style.display = 'block';
                
                if (data.success) {
                    // Success
                    resultSuccess.style.display = 'block';
                    
                    // Set download link
                    if (data.output_filename) {
                        downloadBtn.addEventListener('click', () => {
                            window.location.href = `/download/${data.output_filename}`;
                        });
                    }
                } else {
                    // Error
                    resultError.style.display = 'block';
                    errorMessage.textContent = data.message || 'An error occurred during conversion';
                }
            }, 500);
        })
        .catch(error => {
            clearInterval(progressInterval);
            progressContainer.style.display = 'none';
            resultContainer.style.display = 'block';
            resultError.style.display = 'block';
            errorMessage.textContent = 'Network error: Could not connect to server';
        });
    }
    
    // Update progress bar
    function updateProgress(value) {
        const roundedValue = Math.min(100, Math.round(value));
        progress.style.width = `${roundedValue}%`;
        progressText.textContent = `${roundedValue}%`;
    }
    
    // Try again button event
    tryAgainBtn.addEventListener('click', resetUI);
});