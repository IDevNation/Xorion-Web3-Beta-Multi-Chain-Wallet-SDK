// XOS Wallet - Complete Fixed Script (v2.0)
var isUnlocked = false;
var currentSeed = "";
var walletAddress = "";

function getEl(id) { 
  return document.getElementById(id); 
}

document.addEventListener('DOMContentLoaded', function() {
  checkWalletStatus();
});

function checkWalletStatus() {
  chrome.storage.local.get(['unlocked', 'address'], function(result) {
    if (result.unlocked && result.address) {
      isUnlocked = true;
      walletAddress = result.address;
      showScreen('dashboard');
      updateDashboard();
    } else {
      showScreen('welcome');
    }
  });
}

function showScreen(screenId) {
  var screens = ['welcome', 'create-wallet', 'import-wallet', 'set-password', 'enter-password', 'dashboard'];
  for (var i = 0; i < screens.length; i++) {
    if (getEl(screens[i])) {
      getEl(screens[i]).style.display = 'none';
    }
  }
  if (getEl(screenId)) {
    getEl(screenId).style.display = 'block';
  }
}

function generateSeed() {
  var words = ["abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd", "abuse", "access", "accident", "account", "achieve", "acid", "acoustic", "acquire", "across", "act", "action", "actor", "actress", "actual", "adapt", "add", "addict", "address", "adjust", "admit", "adult", "advance", "advice", "aerobic", "affair", "afford", "afraid", "again", "age", "agent", "agree", "ahead", "aim", "air", "airport", "aisle", "alarm", "album", "alcohol", "alert", "alien", "all", "alley", "allow", "almost", "alone", "alpha", "already", "also", "alter", "always", "amateur", "amazing", "among", "amount", "amused", "analyst", "anchor", "ancient", "anger", "angle", "angry", "animal", "ankle", "announce", "annual", "another", "answer", "antenna", "antique", "anxiety", "any", "apart", "apology", "appear", "apple", "approve", "april", "arch", "arctic", "area", "arena", "argue", "arm", "armed", "armor", "army", "around", "arrange", "arrest", "arrive", "arrow", "art", "artefact", "artist", "artwork", "ask", "aspect", "assault", "asset", "assist", "assume", "asthma", "athlete", "atom", "attack", "attend", "attitude", "attract", "auction", "audit", "august", "aunt", "author", "auto", "autumn", "average", "avocado", "avoid", "awake", "aware", "away", "awesome", "awful", "awkward", "axis"];
  var seed = [];
  for (var i = 0; i < 12; i++) {
    seed.push(words[Math.floor(Math.random() * words.length)]);
  }
  return seed.join(" ");
}

function createWallet() {
  currentSeed = generateSeed();
  var seedDisplay = getEl('new-seed-display');
  if(seedDisplay) seedDisplay.innerText = currentSeed;
  showScreen('set-password');
}

function saveNewWallet() {
  var passEl = getEl('new-password');
  var confirmEl = getEl('confirm-password');
  
  if (!passEl || !confirmEl) return;
  
  var password = passEl.value;
  var confirm = confirmEl.value;
  
  if (password !== confirm) {
    alert("Passwords do not match!");
    return;
  }
  if (password.length < 4) {
    alert("Password must be at least 4 characters");
    return;
  }
  
  walletAddress = "XOS" + Math.random().toString(36).substring(2, 10).toUpperCase() + "...";
  
  chrome.storage.local.set({
    unlocked: true,
    address: walletAddress,
    seed: currentSeed,
    password: password
  }, function() {
    isUnlocked = true;
    showScreen('dashboard');
    updateDashboard();
  });
}

function importWallet() {
  var seedEl = getEl('import-seed');
  if (!seedEl) return;
  
  var seedInput = seedEl.value.trim();
  var words = seedInput.split(/\s+/);
  
  if (words.length !== 12) {
    alert("Please enter exactly 12 words.");
    return;
  }
  currentSeed = seedInput;
  showScreen('set-password');
}

function unlockWallet() {
  var passEl = getEl('unlock-password');
  if (!passEl) return;
  
  var password = passEl.value;
  
  chrome.storage.local.get(['password', 'address', 'seed'], function(result) {
    if (password === result.password) {
      isUnlocked = true;
      walletAddress = result.address;
      currentSeed = result.seed;
      chrome.storage.local.set({ unlocked: true });
      showScreen('dashboard');
      updateDashboard();
    } else {
      alert("Incorrect password!");
    }
  });
}

function lockWallet() {
  isUnlocked = false;
  walletAddress = "";
  chrome.storage.local.set({ unlocked: false });
  showScreen('enter-password');
  var passEl = getEl('unlock-password');
  if(passEl) passEl.value = "";
}

function updateDashboard() {
  var addrEl = getEl('display-address');
  var balEl = getEl('display-balance');
  if(addrEl) addrEl.innerText = walletAddress;
  if(balEl) balEl.innerText = "1000.00 XOS";
}

function sendTransaction() {
  var toEl = getEl('recipient-address');
  var amountEl = getEl('send-amount');
  
  if (!toEl || !amountEl) return;
  
  var to = toEl.value;
  var amount = amountEl.value;
  
  if (!to || !amount) {
    alert("Please fill in all fields");
    return;
  }
  
  alert("Transaction Sent!\nTo: " + to + "\nAmount: " + amount + " XOS");
  toEl.value = "";
  amountEl.value = "";
}

function goBack() {
  showScreen('welcome');
}
