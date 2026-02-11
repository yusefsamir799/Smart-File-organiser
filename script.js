/* ═══════════════════════════════════════
   SMART ORGANIZER — WEBSITE SCRIPTS
   onclick & onload driven interactions
   ═══════════════════════════════════════ */

// ─── TERMINAL DEMO DATA ───
const terminalDemos = {
    hero: [
        { text: '$ smart-organizer --path ~/Downloads', color: 'white', delay: 0 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 400 },
        { text: '      Smart File Organizer  v1.1', color: 'cyan', delay: 500 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 600 },
        { text: '', delay: 700 },
        { text: '📁 Target: /home/user/Downloads', color: 'white', delay: 800 },
        { text: '', delay: 900 },
        { text: '  → vacation.jpg         → Images/', color: 'gray', delay: 1000 },
        { text: '  → report.pdf           → Documents/', color: 'gray', delay: 1100 },
        { text: '  → song.mp3             → Music/', color: 'gray', delay: 1200 },
        { text: '  → screenshot.png       → Images/', color: 'gray', delay: 1300 },
        { text: '  → movie.mp4            → Videos/', color: 'gray', delay: 1400 },
        { text: '  → archive.zip          → Archives/', color: 'gray', delay: 1500 },
        { text: '', delay: 1600 },
        { text: '✓ Organized 6 file(s)', color: 'green', delay: 1700 },
        { text: '  See organizer_log.txt for details.', color: 'dimgray', delay: 1800 },
    ],
    basic: [
        { text: '$ smart-organizer --path ~/Downloads', color: 'white', delay: 0 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 300 },
        { text: '      Smart File Organizer  v1.1', color: 'cyan', delay: 400 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 500 },
        { text: '', delay: 600 },
        { text: '📁 Target: /home/user/Downloads', color: 'white', delay: 700 },
        { text: '', delay: 750 },
        { text: '  → photo_001.jpg        → Images/', color: 'gray', delay: 800 },
        { text: '  → photo_002.png        → Images/', color: 'gray', delay: 880 },
        { text: '  → thesis.pdf           → Documents/', color: 'gray', delay: 960 },
        { text: '  → budget.xlsx          → Documents/', color: 'gray', delay: 1040 },
        { text: '  → podcast.mp3          → Music/', color: 'gray', delay: 1120 },
        { text: '  → trailer.mp4          → Videos/', color: 'gray', delay: 1200 },
        { text: '  → backup.zip           → Archives/', color: 'gray', delay: 1280 },
        { text: '  → app.js               → Code/', color: 'gray', delay: 1360 },
        { text: '', delay: 1500 },
        { text: '✓ Organized 8 file(s)', color: 'green', delay: 1600 },
        { text: '  See organizer_log.txt for details.', color: 'dimgray', delay: 1700 },
    ],
    dryrun: [
        { text: '$ smart-organizer --dry-run --path ~/Desktop', color: 'white', delay: 0 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 300 },
        { text: '      Smart File Organizer  v1.1', color: 'cyan', delay: 400 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 500 },
        { text: '', delay: 600 },
        { text: '📋 PREVIEW MODE — no files will be moved', color: 'yellow', delay: 700 },
        { text: '   Remove --dry-run to organize for real.', color: 'yellow', delay: 800 },
        { text: '', delay: 850 },
        { text: '📁 Target: /home/user/Desktop', color: 'white', delay: 900 },
        { text: '', delay: 950 },
        { text: '  → wallpaper.png     [would move] → Images/', color: 'gray', delay: 1000 },
        { text: '  → resume.docx       [would move] → Documents/', color: 'gray', delay: 1080 },
        { text: '  → demo.mp4          [would move] → Videos/', color: 'gray', delay: 1160 },
        { text: '  → notes.txt         [would move] → Documents/', color: 'gray', delay: 1240 },
        { text: '  → style.css         [would move] → Code/', color: 'gray', delay: 1320 },
        { text: '', delay: 1450 },
        { text: '✓ Preview complete: 5 file(s) would be moved', color: 'green', delay: 1550 },
        { text: '   Run again without --dry-run to apply changes.', color: 'yellow', delay: 1650 },
    ],
    duplicates: [
        { text: '$ smart-organizer --find-duplicates --path ~/Files', color: 'white', delay: 0 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 300 },
        { text: '      Smart File Organizer  v1.1', color: 'cyan', delay: 400 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 500 },
        { text: '', delay: 600 },
        { text: '📁 Target: /home/user/Files', color: 'white', delay: 700 },
        { text: '', delay: 750 },
        { text: '  → report.pdf           → Documents/', color: 'gray', delay: 850 },
        { text: '  → report.pdf           ⚠ DUPLICATE (skipped)', color: 'yellow', delay: 950 },
        { text: '  → photo.jpg            → Images/', color: 'gray', delay: 1050 },
        { text: '  → photo.jpg            ⚠ DUPLICATE (skipped)', color: 'yellow', delay: 1150 },
        { text: '  → song.flac            → Music/', color: 'gray', delay: 1250 },
        { text: '  → archive.tar.gz       → Archives/', color: 'gray', delay: 1350 },
        { text: '', delay: 1500 },
        { text: '✓ Organized 4 file(s)', color: 'green', delay: 1600 },
        { text: '   2 duplicate(s) skipped', color: 'yellow', delay: 1700 },
        { text: '  See organizer_log.txt for details.', color: 'dimgray', delay: 1800 },
    ],
    full: [
        { text: '$ smart-organizer --dry-run --find-duplicates --keep-structure --path ~/Projects', color: 'white', delay: 0 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 300 },
        { text: '      Smart File Organizer  v1.1', color: 'cyan', delay: 400 },
        { text: '═══════════════════════════════════════', color: 'cyan', delay: 500 },
        { text: '', delay: 600 },
        { text: '📋 PREVIEW MODE — no files will be moved', color: 'yellow', delay: 700 },
        { text: '   Remove --dry-run to organize for real.', color: 'yellow', delay: 800 },
        { text: '', delay: 850 },
        { text: '📁 Target: /home/user/Projects', color: 'white', delay: 900 },
        { text: '', delay: 950 },
        { text: '  → web/index.html       [would move] → Code/web/', color: 'gray', delay: 1050 },
        { text: '  → web/style.css        [would move] → Code/web/', color: 'gray', delay: 1150 },
        { text: '  → design/logo.png      [would move] → Images/design/', color: 'gray', delay: 1250 },
        { text: '  → design/logo.png      ⚠ DUPLICATE (would skip)', color: 'yellow', delay: 1350 },
        { text: '  → docs/readme.pdf      [would move] → Documents/docs/', color: 'gray', delay: 1450 },
        { text: '  → assets/intro.mp4     [would move] → Videos/assets/', color: 'gray', delay: 1550 },
        { text: '', delay: 1700 },
        { text: '✓ Preview complete: 5 file(s) would be moved', color: 'green', delay: 1800 },
        { text: '   1 duplicate(s) detected', color: 'yellow', delay: 1900 },
        { text: '   Run again without --dry-run to apply changes.', color: 'yellow', delay: 2000 },
    ],
};

// Color map for terminal text
const colorMap = {
    white: '#d6e0f0',
    cyan: '#60a5fa',
    green: '#4ade80',
    yellow: '#facc15',
    red: '#f87171',
    gray: '#8899b3',
    dimgray: '#4a6080',
};

// ─── TERMINAL TYPING ENGINE ───
function typeTerminal(targetId, demoKey) {
    const output = document.getElementById(targetId);
    if (!output) return;

    output.innerHTML = '';
    const lines = terminalDemos[demoKey];
    let timeouts = [];

    lines.forEach((line, index) => {
        const timeout = setTimeout(() => {
            const span = document.createElement('span');
            span.style.color = colorMap[line.color] || colorMap.gray;
            span.textContent = line.text;

            // Typing effect for first line (the command)
            if (index === 0) {
                span.textContent = '';
                output.appendChild(span);
                output.appendChild(document.createTextNode('\n'));

                let charIdx = 0;
                const typeChar = () => {
                    if (charIdx < line.text.length) {
                        span.textContent += line.text[charIdx];
                        charIdx++;
                        setTimeout(typeChar, 18);
                    }
                };
                typeChar();
            } else {
                output.appendChild(span);
                output.appendChild(document.createTextNode('\n'));
            }

            // Auto-scroll
            output.parentElement.scrollTop = output.parentElement.scrollHeight;
        }, line.delay);

        timeouts.push(timeout);
    });

    // Store timeouts so we can clear them
    output._timeouts = timeouts;
}

function clearTerminalTimeouts(targetId) {
    const output = document.getElementById(targetId);
    if (output && output._timeouts) {
        output._timeouts.forEach(clearTimeout);
        output._timeouts = [];
    }
}

// ─── ONCLICK HANDLERS ───

// Smooth scroll to section
function smoothScroll(event, targetId) {
    event.preventDefault();
    const target = document.getElementById(targetId);
    if (target) {
        const navHeight = document.getElementById('navbar').offsetHeight;
        const top = target.getBoundingClientRect().top + window.scrollY - navHeight - 20;
        window.scrollTo({ top, behavior: 'smooth' });
    }
    // Close mobile menu if open
    document.querySelector('.nav-links')?.classList.remove('open');
}

// Scroll to top
function scrollToTop() {
    window.scrollTo({ top: 0, behavior: 'smooth' });
}

// Toggle mobile menu
function toggleMenu() {
    document.querySelector('.nav-links')?.classList.toggle('open');
}

// Run demo terminal
function runDemo(button, demoKey) {
    // Update active button
    document.querySelectorAll('.cmd-btn').forEach(btn => btn.classList.remove('active'));
    button.classList.add('active');

    // Clear previous and run new demo
    clearTerminalTimeouts('demo-output');
    typeTerminal('demo-output', demoKey);
}

// Copy code block
function copyCode(element) {
    const code = element.querySelector('code').textContent;
    navigator.clipboard.writeText(code).then(() => {
        element.classList.add('copied');
        const hint = element.querySelector('.copy-hint');
        const original = hint.textContent;
        hint.textContent = '✓ Copied!';
        setTimeout(() => {
            element.classList.remove('copied');
            hint.textContent = original;
        }, 2000);
    }).catch(() => {
        // Fallback for older browsers
        const textarea = document.createElement('textarea');
        textarea.value = code;
        document.body.appendChild(textarea);
        textarea.select();
        document.execCommand('copy');
        document.body.removeChild(textarea);

        element.classList.add('copied');
        const hint = element.querySelector('.copy-hint');
        hint.textContent = '✓ Copied!';
        setTimeout(() => {
            element.classList.remove('copied');
            hint.textContent = 'Click to copy';
        }, 2000);
    });
}

// Feature card expand
function expandFeature(card) {
    const wasExpanded = card.classList.contains('expanded');
    document.querySelectorAll('.feature-card').forEach(c => c.classList.remove('expanded'));
    if (!wasExpanded) {
        card.classList.add('expanded');
    }
}

// Track download clicks (placeholder)
function trackClick(label) {
    console.log('[Smart Organizer] Click tracked:', label);
}

// ─── COUNTER ANIMATION ───
function animateCounter(elementId, target, duration) {
    const el = document.getElementById(elementId);
    if (!el) return;

    const start = 0;
    const startTime = performance.now();

    function update(currentTime) {
        const elapsed = currentTime - startTime;
        const progress = Math.min(elapsed / duration, 1);
        // Ease-out cubic
        const eased = 1 - Math.pow(1 - progress, 3);
        const current = Math.floor(start + (target - start) * eased);

        el.textContent = current.toLocaleString();

        if (progress < 1) {
            requestAnimationFrame(update);
        } else {
            el.textContent = target.toLocaleString() + '+';
        }
    }

    requestAnimationFrame(update);
}

// ─── SCROLL OBSERVER ───
function setupScrollObserver() {
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
            }
        });
    }, { threshold: 0.1, rootMargin: '0px 0px -60px 0px' });

    document.querySelectorAll('.feature-card, .install-step, .transform-card, .download-btn, .pricing-card, .trust-message').forEach(el => {
        el.classList.add('fade-in-view');
        observer.observe(el);
    });
}

// ─── NAVBAR SCROLL EFFECT ───
function setupNavbarScroll() {
    const navbar = document.getElementById('navbar');
    let ticking = false;

    window.addEventListener('scroll', () => {
        if (!ticking) {
            requestAnimationFrame(() => {
                if (window.scrollY > 50) {
                    navbar.classList.add('scrolled');
                } else {
                    navbar.classList.remove('scrolled');
                }
                ticking = false;
            });
            ticking = true;
        }
    });
}

// ─── PRICING HANDLERS ───

function selectPlan(card, plan) {
    // Visual feedback
    document.querySelectorAll('.pricing-card').forEach(c => c.style.transform = '');
}

function setAmount(event, amount) {
    event.stopPropagation();
    const input = document.getElementById('supporter-amount');
    input.value = amount;

    // Update active button
    document.querySelectorAll('.quick-amounts button').forEach(btn => btn.classList.remove('active'));
    event.target.classList.add('active');
}

function updateAmount(input) {
    const val = parseInt(input.value);
    if (val < 1) input.value = 1;
    if (val > 999) input.value = 999;

    // Update active quick-amount button
    document.querySelectorAll('.quick-amounts button').forEach(btn => {
        const btnVal = parseInt(btn.textContent.replace('$', ''));
        btn.classList.toggle('active', btnVal === parseInt(input.value));
    });
}

function handleSupport(event) {
    event.preventDefault();
    event.stopPropagation();
    const amount = document.getElementById('supporter-amount').value;
    
    // Create thank you modal
    const modal = document.createElement('div');
    modal.style.cssText = `
        position: fixed; inset: 0; z-index: 9999;
        background: rgba(1, 4, 9, 0.9); backdrop-filter: blur(12px);
        display: flex; align-items: center; justify-content: center;
        animation: fadeIn 0.3s ease-out;
    `;
    modal.innerHTML = `
        <div style="
            background: #0a1628; border: 1px solid #112240;
            border-radius: 20px; padding: 48px 40px; text-align: center;
            max-width: 440px; width: 90%; position: relative;
            box-shadow: 0 40px 80px rgba(0,0,0,0.5);
        ">
            <div style="font-size: 3rem; margin-bottom: 16px;">💛</div>
            <h3 style="font-size: 1.5rem; font-weight: 800; margin-bottom: 12px; color: #fbbf24;">
                Thank you!
            </h3>
            <p style="color: #8899b3; font-size: 1rem; line-height: 1.7; margin-bottom: 8px;">
                Your <strong style="color: #fbbf24;">$${amount}</strong> means the world to us.
            </p>
            <p style="color: #4a6080; font-size: 0.88rem; line-height: 1.6; margin-bottom: 28px;">
                You're helping keep Smart Organizer free for everyone. 
                People like you make open source possible.
            </p>
            <button onclick="this.closest('div').parentElement.remove()" style="
                padding: 12px 32px; background: linear-gradient(135deg, #f59e0b, #d97706);
                border: none; border-radius: 10px; color: #1a1a1a;
                font-weight: 700; font-size: 0.95rem; cursor: pointer;
                font-family: 'Outfit', sans-serif;
            ">Continue to Download</button>
        </div>
    `;
    document.body.appendChild(modal);
    modal.addEventListener('click', (e) => {
        if (e.target === modal) modal.remove();
    });
}

// ─── ONLOAD ───
window.onload = function () {
    console.log('[Smart Organizer] Website loaded');

    // 1. Start hero terminal animation
    typeTerminal('terminal-output', 'hero');

    // 2. Start the demo section with default
    typeTerminal('demo-output', 'basic');

    // 3. Animate the stats counter
    animateCounter('stat-files', 10000, 2000);

    // 4. Setup scroll-based animations
    setupScrollObserver();

    // 5. Setup navbar scroll effect
    setupNavbarScroll();

    // 6. Add fade-in delays to feature cards
    document.querySelectorAll('.feature-card').forEach((card, i) => {
        card.style.transitionDelay = `${i * 0.1}s`;
    });
};
